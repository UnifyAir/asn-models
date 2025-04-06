#[allow(unused_imports)]
mod common;
pub use common::*;
mod ies;
pub use ies::*;
mod pdu;
pub use pdu::*;
mod top_pdu;
pub use top_pdu::*;
mod conversion;
use asn1_per::{CodecDataAllocator, PerCodec};

impl From<ErrorCriticality> for Criticality {
	fn from(e: ErrorCriticality) -> Self {
		match e {
			ErrorCriticality::Reject => Criticality::Reject,
			ErrorCriticality::Ignore => Criticality::Ignore,
			ErrorCriticality::Notify => Criticality::Notify,
		}
	}
}

impl From<ErrorTypeOfError> for TypeOfError {
	fn from(e: ErrorTypeOfError) -> Self {
		match e {
			ErrorTypeOfError::NotUnderstood => TypeOfError::NotUnderstood,
			ErrorTypeOfError::Missing => TypeOfError::Missing,
		}
	}
}

impl From<ErrorDiagnostic> for CriticalityDiagnosticsIeItem {
	fn from(e: ErrorDiagnostic) -> Self {
		CriticalityDiagnosticsIeItem {
			ie_criticality: e.ie_criticality.into(),
			ie_id: ProtocolIeId(e.ie_id),
			type_of_error: e.type_of_error.into(),
		}
	}
}

// TODO: Make this function zero copy, as internally both types are same. https://stackoverflow.com/a/48309116.
// Add compiler error for checking if the type layouts and alignments are also
// same.
pub fn convert_diagnostics_to_ie(
	diagnostics: Vec<ErrorDiagnostic>
) -> Option<CriticalityDiagnosticsIeList> {
	let diagnostics = diagnostics
		.into_iter()
		.map(|d| d.into())
		.collect::<Vec<_>>();
	let list = NonEmpty::from_vec(diagnostics);
	list.map(|list| CriticalityDiagnosticsIeList(list))
}

impl NgapPdu {
	// TODO: Support for the Partial Decoding in NgapPdu. To prevent refetching
	// the same bytes again and again.
	pub fn get_message_info(
		bytes: &[u8]
	) -> (
		Option<TriggeringMessage>,
		Option<ProcedureCode>,
		Option<Criticality>,
	) {
		let mut data = <NgapPdu as PerCodec>::Allocator::from_slice(bytes);
		let mut triggering_message = None;
		let mut procedure_code = None;
		let mut criticality = None;

		// Macro equivalent of ? operator
		macro_rules! try_or_return {
			($expr:expr) => {
				match $expr {
					Ok(val) => val,
					Err(_) => return (triggering_message, procedure_code, criticality),
				}
			};
		}

		let (idx, _) = try_or_return!(decode::decode_choice_idx(&mut data, 0, 2, true));

		triggering_message = match idx {
			0 => Some(TriggeringMessage::InitiatingMessage),
			1 => Some(TriggeringMessage::SuccessfulOutcome),
			2 => Some(TriggeringMessage::UnsuccessfulOutcome),
			_ => return (None, None, None),
		};

		let (code, _) =
			try_or_return!(decode::decode_integer(&mut data, Some(0), Some(255), false));
		procedure_code = Some(ProcedureCode(code as u8));

		criticality = Some(try_or_return!(Criticality::decode(&mut data)));

		(triggering_message, procedure_code, criticality)
	}
}

#[derive(Debug, Hash)]
pub enum NgapSignallingId {
	AmfUeNgapId(AmfUeNgapId),
	RanUeNgapId(RanUeNgapId),
	AmfRanUeNgapId(AmfUeNgapId, RanUeNgapId),
	UeEmpty,
	NonUeEmpty,
}

pub enum NgapSignallingMode {
	Ue,
	NonUe,
	Both,
}

pub trait Id<T: Hash> {
	const MODE: NgapSignallingMode;
	fn get_id(&self) -> T;
}
