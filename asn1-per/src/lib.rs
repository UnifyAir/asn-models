pub mod error;
mod transaction;
pub use asn1_codecs::{PerCodecData, PerCodecError};
pub use bitvec::prelude::*;
// pub type BitString = BitVec<u8, Msb0>;
pub use error::Error as ThreeGppAsn1PerError;
pub use nonempty::*;
pub use num_enum::TryFromPrimitive;
pub use transaction::*;
mod bitstring;
pub use bitstring::BitString;

pub trait PerCodec: Sized {
	type Allocator: CodecDataAllocator;
	fn decode(data: &mut PerCodecData) -> Result<Self, ThreeGppAsn1PerError>;
	fn encode(
		&self,
		_data: &mut PerCodecData,
	) -> Result<(), ThreeGppAsn1PerError>;
}

pub trait SerDes: Sized {
	fn into_bytes(self) -> Result<Vec<u8>, ThreeGppAsn1PerError>;
	fn from_bytes(bytes: &[u8]) -> Result<Self, ThreeGppAsn1PerError>;
}

impl<T: PerCodec> SerDes for T {
	fn into_bytes(self) -> Result<Vec<u8>, ThreeGppAsn1PerError> {
		let mut d = T::Allocator::new_codec_data();
		self.encode(&mut d)?;
		Ok(d.into_bytes())
	}

	fn from_bytes(bytes: &[u8]) -> Result<Self, ThreeGppAsn1PerError> {
		let mut d = T::Allocator::from_slice(bytes);
		Self::decode(&mut d)
	}
}

pub trait CodecDataAllocator {
	fn new_codec_data() -> PerCodecData;
	fn from_slice(bytes: &[u8]) -> PerCodecData;
}

pub mod aper {
	pub use asn1_codecs::aper::{decode, encode};

	pub use super::{CodecDataAllocator, PerCodecData};
	pub struct Allocator;
	impl CodecDataAllocator for Allocator {
		fn new_codec_data() -> PerCodecData {
			PerCodecData::new_aper()
		}
		fn from_slice(bytes: &[u8]) -> PerCodecData {
			crate::PerCodecData::from_slice_aper(bytes)
		}
	}
}

pub mod uper {
	pub use asn1_codecs::uper::{decode, encode};

	pub use super::{CodecDataAllocator, PerCodecData};

	pub struct Allocator;
	impl CodecDataAllocator for Allocator {
		fn new_codec_data() -> PerCodecData {
			PerCodecData::new_uper()
		}
		fn from_slice(bytes: &[u8]) -> PerCodecData {
			crate::PerCodecData::from_slice_uper(bytes)
		}
	}
}
