//! transactions - allows definition of procedures using individual ASN.1 requests and responses

use crate::{PerCodecError, SerDes};
use anyhow::Result;
use async_channel::RecvError;
use async_trait::async_trait;
use tracing::{debug, warn}; 
use std::{fmt::Debug, future::Future, pin::Pin};
use thiserror::Error;
use crate::ThreeGppAsn1PerError;

#[async_trait]
pub trait Procedure {
    const CODE: u8;
    type TopPdu: SerDes + Send + Sync + 'static;
    type Request: Send + Sync + 'static + Debug;
    type Success;
    type Failure;
    #[cfg(feature = "gnb")]
    fn encode_request(r: Self::Request) -> Result<Vec<u8>, ThreeGppAsn1PerError>;
    #[cfg(feature = "gnb")]
    fn decode_response(bytes: &[u8]) -> Result<Self::Success, RequestError<Self::Failure>>;
    #[cfg(feature = "amf")]
    fn decode_request(bytes: &[u8]) -> Result<Self::Request, ThreeGppAsn1PerError>;
    async fn call_provider<T: RequestProvider<Self>>(
        provider: &T,
        req: Self::Request,
    ) -> Option<ResponseAction<Self::TopPdu>>;
}

#[async_trait]
pub trait Indication {
    const CODE: u8;
    type TopPdu: SerDes + Send + Sync + 'static;
    type Request: Send + Sync + 'static + Debug;
    #[cfg(feature = "gnb")]
    fn encode_request(r: Self::Request) -> Result<Vec<u8>, ThreeGppAsn1PerError>;
    #[cfg(feature = "amf")]
    fn decode_request(bytes: &[u8]) -> Result<Self::Request, ThreeGppAsn1PerError>;
    async fn call_provider<T: IndicationHandler<Self>>(
        provider: &T,
        req: Self::Request,
    );
}

pub type ResponseAction<T> = (T, Option<Pin<Box<dyn Future<Output = ()> + Send>>>);

#[derive(Error, Debug)]
pub enum RequestError<U> {
    #[error("Unsuccessful outcome")]
    UnsuccessfulOutcome(U),
    #[error("{0}")]
    Other(String),
}

impl<T> From<PerCodecError> for RequestError<T> {
    fn from(e: PerCodecError) -> Self {
        RequestError::Other(format!("Codec error: {:?}", e))
    }
}

impl<T> From<ThreeGppAsn1PerError> for RequestError<T> {
    fn from(e: ThreeGppAsn1PerError) -> Self {
        RequestError::Other(format!("Codec error: {:?}", e.codec_error))
    }
}

impl<T> From<RecvError> for RequestError<T> {
    fn from(e: RecvError) -> Self {
        RequestError::Other(format!("Channel recv error: {:?}", e))
    }
}

impl<T> From<anyhow::Error> for RequestError<T> {
    fn from(e: anyhow::Error) -> Self {
        RequestError::Other(format!("Transport error: {:?}", e))
    }
}



/// Trait representing the ability to handle a single procedure.
#[async_trait]
pub trait RequestProvider<P: Procedure + ?Sized>: Send + Sync {
    async fn request(
        &self,
        r: P::Request,
    ) -> Result<ResponseAction<P::Success>, RequestError<P::Failure>> {
        debug!("Received unimplemented request {:?}", r);
        Err(RequestError::Other("Not implemented".to_string()))
    }
}

/// Trait representing the ability to handle an indication.
#[async_trait]
pub trait IndicationHandler<I: Indication + ?Sized>: Send + Sync {
    async fn handle(&self, i: I::Request) {
        warn!("Received unimplemented indication {:?}", i);
    }
}

/// Trait representing the ability to handle multiple procedures that use the same top level PDU.
#[async_trait]
pub trait InterfaceProvider: Send + Sync {
    type TopPdu: SerDes;
    async fn route_request(
        &self,
        p: Self::TopPdu,
    ) -> Option<ResponseAction<Self::TopPdu>>;
}

/// Trait representing the ability to handle and respond to a request in wire format.
#[async_trait]
pub trait RequestMessageHandler: Send + Sync {
    async fn handle_request(
        &self,
        message: &[u8],
    ) -> Option<ResponseAction<Vec<u8>>>;
}

// An interface provider is a request message handler.
#[async_trait]
impl<T: SerDes + Send + Sync, I: InterfaceProvider<TopPdu = T>> RequestMessageHandler for I {
    async fn handle_request(
        &self,
        message: &[u8],
    ) -> Option<ResponseAction<Vec<u8>>> {
        let pdu = match T::from_bytes(message) {
            Ok(p) => p,
            Err(e) => {
                warn!("ASN.1 decode failed - {:?}", e);
                return None;
            }
        };
        match self
            .route_request(pdu)
            .await
            .map(|(m, a)| (m.into_bytes(), a))
        {
            None => None,
            Some((Ok(bytes), a)) => Some((bytes, a)),
            Some((Err(e), _)) => {
                warn!("ASN.1 encode failed - {:?}", e);
                None
            }
        }
    }
}
