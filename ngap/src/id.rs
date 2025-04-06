use crate::{
	Id,
	InitiatingMessage,
	NgapPdu,
	NgapSignallingMode,
	SuccessfulOutcome,
	NgapSignallingId,
	UnsuccessfulOutcome,
	pdu::*,
};

// Macro to implement Id trait for PDUs
macro_rules! impl_id_for_pdu {
	// For UE signalling PDUs with both AMF and RAN UE IDs
	($type:ty, ue_signalling, amf_ran) => {
		impl Id<NgapSignallingId> for $type {
			const MODE: NgapSignallingMode = NgapSignallingMode::Ue;
			fn get_id(&self) -> NgapSignallingId {
				NgapSignallingId::AmfRanUeNgapId(self.amf_ue_ngap_id, self.ran_ue_ngap_id)
			}
		}
	};

	// For UE signalling PDUs with multiple Ue IDs
	($type:ty, ue_signalling, empty) => {
		impl Id<NgapSignallingId> for $type {
			const MODE: NgapSignallingMode = NgapSignallingMode::Ue;
			fn get_id(&self) -> NgapSignallingId {
				NgapSignallingId::UeEmpty
			}
		}
	};

	($type:ty, ue_signalling, amf_ran_optional) => {
		impl Id<NgapSignallingId> for $type {
			const MODE: NgapSignallingMode = NgapSignallingMode::Ue;
			fn get_id(&self) -> NgapSignallingId {
				match (self.amf_ue_ngap_id, self.ran_ue_ngap_id) {
					(Some(amf_ue_ngap_id), Some(ran_ue_ngap_id)) => {
						NgapSignallingId::AmfRanUeNgapId(amf_ue_ngap_id, ran_ue_ngap_id)
					}
					(Some(amf_ue_ngap_id), None) => NgapSignallingId::AmfUeNgapId(amf_ue_ngap_id),
					(None, Some(ran_ue_ngap_id)) => NgapSignallingId::RanUeNgapId(ran_ue_ngap_id),
					(None, None) => NgapSignallingId::UeEmpty,
				}
			}
		}
	};

	($type:ty, ue_signalling, amf_ran_optional_amf) => {
		impl Id<NgapSignallingId> for $type {
			const MODE: NgapSignallingMode = NgapSignallingMode::Ue;
			fn get_id(&self) -> NgapSignallingId {
				match self.amf_ue_ngap_id {
					Some(amf_ue_ngap_id) => {
						NgapSignallingId::AmfRanUeNgapId(amf_ue_ngap_id, self.ran_ue_ngap_id)
					}
					None => NgapSignallingId::RanUeNgapId(self.ran_ue_ngap_id),
				}
			}
		}
	};
	// For UE signalling PDUs with only AMF UE ID
	($type:ty, ue_signalling, amf) => {
		impl Id<NgapSignallingId> for $type {
			const MODE: NgapSignallingMode = NgapSignallingMode::Ue;
			fn get_id(&self) -> NgapSignallingId {
				NgapSignallingId::AmfUeNgapId(self.amf_ue_ngap_id)
			}
		}
	};

	// For UE signalling PDUs with only RAN UE ID
	($type:ty, ue_signalling, ran) => {
		impl Id<NgapSignallingId> for $type {
			const MODE: NgapSignallingMode = NgapSignallingMode::Ue;
			fn get_id(&self) -> NgapSignallingId {
				NgapSignallingId::RanUeNgapId(self.ran_ue_ngap_id)
			}
		}
	};

	// For non-UE signalling PDUs
	($type:ty, non_ue_signalling) => {
		impl Id<NgapSignallingId> for $type {
			const MODE: NgapSignallingMode = NgapSignallingMode::NonUe;
			fn get_id(&self) -> NgapSignallingId {
				NgapSignallingId::NonUeEmpty
			}
		}
	};
}

// Implement for InitiatingMessage PDUs
impl_id_for_pdu!(AmfConfigurationUpdate, non_ue_signalling);
impl_id_for_pdu!(AmfcpRelocationIndication, ue_signalling, amf_ran);
impl_id_for_pdu!(AmfStatusIndication, non_ue_signalling);
impl_id_for_pdu!(BroadcastSessionModificationRequest, non_ue_signalling);
impl_id_for_pdu!(BroadcastSessionReleaseRequest, non_ue_signalling);
impl_id_for_pdu!(BroadcastSessionReleaseRequired, non_ue_signalling);
impl_id_for_pdu!(BroadcastSessionSetupRequest, non_ue_signalling);
impl_id_for_pdu!(CellTrafficTrace, ue_signalling, amf_ran);
impl_id_for_pdu!(ConnectionEstablishmentIndication, ue_signalling, ran);
impl_id_for_pdu!(DeactivateTrace, ue_signalling, amf_ran);
impl_id_for_pdu!(DistributionSetupRequest, non_ue_signalling);
impl_id_for_pdu!(DistributionReleaseRequest, non_ue_signalling);
impl_id_for_pdu!(DownlinkNasTransport, ue_signalling, amf_ran);
impl_id_for_pdu!(DownlinkNonUeAssociatedNrPPaTransport, non_ue_signalling);
impl_id_for_pdu!(DownlinkRanConfigurationTransfer, non_ue_signalling);
impl_id_for_pdu!(DownlinkRanEarlyStatusTransfer, ue_signalling, amf_ran);
impl_id_for_pdu!(DownlinkRanStatusTransfer, ue_signalling, amf_ran);
impl_id_for_pdu!(DownlinkUeAssociatedNrPPaTransport, ue_signalling, amf_ran);
impl_id_for_pdu!(HandoverCancel, ue_signalling, amf_ran);
impl_id_for_pdu!(HandoverNotify, ue_signalling, amf_ran);
impl_id_for_pdu!(HandoverRequired, ue_signalling, amf_ran);
impl_id_for_pdu!(HandoverRequest, ue_signalling, amf);
impl_id_for_pdu!(HandoverSuccess, ue_signalling, amf_ran);
impl_id_for_pdu!(InitialContextSetupRequest, ue_signalling, amf_ran);
impl_id_for_pdu!(InitialUeMessage, ue_signalling, ran);
impl_id_for_pdu!(LocationReport, ue_signalling, amf_ran);
impl_id_for_pdu!(LocationReportingControl, ue_signalling, amf_ran);
impl_id_for_pdu!(LocationReportingFailureIndication, ue_signalling, amf_ran);
impl_id_for_pdu!(MulticastSessionActivationRequest, non_ue_signalling);
impl_id_for_pdu!(MulticastSessionDeactivationRequest, non_ue_signalling);
impl_id_for_pdu!(MulticastSessionUpdateRequest, non_ue_signalling);
impl_id_for_pdu!(MulticastGroupPaging, non_ue_signalling);
impl_id_for_pdu!(NasNonDeliveryIndication, ue_signalling, amf_ran);
impl_id_for_pdu!(NgReset, non_ue_signalling);
impl_id_for_pdu!(NgSetupRequest, non_ue_signalling);
impl_id_for_pdu!(OverloadStart, non_ue_signalling);
impl_id_for_pdu!(OverloadStop, non_ue_signalling);
impl_id_for_pdu!(Paging, non_ue_signalling);
impl_id_for_pdu!(PathSwitchRequest, ue_signalling, ran);
impl_id_for_pdu!(PduSessionResourceModifyRequest, ue_signalling, amf_ran);
impl_id_for_pdu!(PduSessionResourceModifyIndication, ue_signalling, amf_ran);
impl_id_for_pdu!(PduSessionResourceNotify, ue_signalling, amf_ran);
impl_id_for_pdu!(PduSessionResourceReleaseCommand, ue_signalling, amf_ran);
impl_id_for_pdu!(PduSessionResourceSetupRequest, ue_signalling, amf_ran);
impl_id_for_pdu!(PwsCancelRequest, non_ue_signalling);
impl_id_for_pdu!(PwsFailureIndication, non_ue_signalling);
impl_id_for_pdu!(PwsRestartIndication, non_ue_signalling);
impl_id_for_pdu!(RanConfigurationUpdate, non_ue_signalling);
impl_id_for_pdu!(RancpRelocationIndication, ue_signalling, ran);
impl_id_for_pdu!(RerouteNasRequest, ue_signalling, amf_ran_optional_amf);
impl_id_for_pdu!(RetrieveUeInformation, non_ue_signalling);
impl_id_for_pdu!(RrcInactiveTransitionReport, ue_signalling, amf_ran);
impl_id_for_pdu!(SecondaryRatDataUsageReport, ue_signalling, amf_ran);
impl_id_for_pdu!(TraceFailureIndication, ue_signalling, amf_ran);
impl_id_for_pdu!(TraceStart, ue_signalling, amf_ran);
impl_id_for_pdu!(UeContextModificationRequest, ue_signalling, amf_ran);
impl_id_for_pdu!(UeContextReleaseCommand, ue_signalling, empty);
impl_id_for_pdu!(UeContextReleaseRequest, ue_signalling, amf_ran);
impl_id_for_pdu!(UeContextResumeRequest, ue_signalling, amf_ran);
impl_id_for_pdu!(UeContextSuspendRequest, ue_signalling, amf_ran);
impl_id_for_pdu!(UeInformationTransfer, non_ue_signalling);
impl_id_for_pdu!(UeRadioCapabilityCheckRequest, ue_signalling, amf_ran);
impl_id_for_pdu!(UeRadioCapabilityIdMappingRequest, non_ue_signalling);
impl_id_for_pdu!(UeRadioCapabilityInfoIndication, ue_signalling, amf_ran);
impl_id_for_pdu!(UeTnlaBindingReleaseRequest, ue_signalling, amf_ran);
impl_id_for_pdu!(UplinkNasTransport, ue_signalling, amf_ran);
impl_id_for_pdu!(UplinkNonUeAssociatedNrPPaTransport, non_ue_signalling);
impl_id_for_pdu!(UplinkRanConfigurationTransfer, non_ue_signalling);
impl_id_for_pdu!(UplinkRanEarlyStatusTransfer, ue_signalling, amf_ran);
impl_id_for_pdu!(UplinkRanStatusTransfer, ue_signalling, amf_ran);
impl_id_for_pdu!(UplinkUeAssociatedNrPPaTransport, ue_signalling, amf_ran);
impl_id_for_pdu!(WriteReplaceWarningRequest, non_ue_signalling);
impl_id_for_pdu!(UplinkRimInformationTransfer, non_ue_signalling);
impl_id_for_pdu!(DownlinkRimInformationTransfer, non_ue_signalling);

// Implement for SuccessfulOutcome PDUs
impl_id_for_pdu!(AmfConfigurationUpdateAcknowledge, non_ue_signalling);
impl_id_for_pdu!(BroadcastSessionModificationResponse, non_ue_signalling);
impl_id_for_pdu!(BroadcastSessionReleaseResponse, non_ue_signalling);
impl_id_for_pdu!(BroadcastSessionSetupResponse, non_ue_signalling);
impl_id_for_pdu!(DistributionSetupResponse, non_ue_signalling);
impl_id_for_pdu!(DistributionReleaseResponse, non_ue_signalling);
impl_id_for_pdu!(HandoverCancelAcknowledge, ue_signalling, amf_ran);
impl_id_for_pdu!(HandoverCommand, ue_signalling, amf_ran);
impl_id_for_pdu!(HandoverRequestAcknowledge, ue_signalling, amf_ran);
impl_id_for_pdu!(InitialContextSetupResponse, ue_signalling, amf_ran);
impl_id_for_pdu!(MulticastSessionActivationResponse, non_ue_signalling);
impl_id_for_pdu!(MulticastSessionDeactivationResponse, non_ue_signalling);
impl_id_for_pdu!(MulticastSessionUpdateResponse, non_ue_signalling);
impl_id_for_pdu!(NgResetAcknowledge, non_ue_signalling);
impl_id_for_pdu!(NgSetupResponse, non_ue_signalling);
impl_id_for_pdu!(PathSwitchRequestAcknowledge, ue_signalling, amf_ran);
impl_id_for_pdu!(PduSessionResourceModifyResponse, ue_signalling, amf_ran);
impl_id_for_pdu!(PduSessionResourceModifyConfirm, ue_signalling, amf_ran);
impl_id_for_pdu!(PduSessionResourceReleaseResponse, ue_signalling, amf_ran);
impl_id_for_pdu!(PduSessionResourceSetupResponse, ue_signalling, amf_ran);
impl_id_for_pdu!(PwsCancelResponse, non_ue_signalling);
impl_id_for_pdu!(RanConfigurationUpdateAcknowledge, non_ue_signalling);
impl_id_for_pdu!(UeContextModificationResponse, ue_signalling, amf_ran);
impl_id_for_pdu!(UeContextReleaseComplete, ue_signalling, amf_ran);
impl_id_for_pdu!(UeContextResumeResponse, ue_signalling, amf_ran);
impl_id_for_pdu!(UeContextSuspendResponse, ue_signalling, amf_ran);
impl_id_for_pdu!(UeRadioCapabilityCheckResponse, ue_signalling, amf_ran);
impl_id_for_pdu!(UeRadioCapabilityIdMappingResponse, non_ue_signalling);
impl_id_for_pdu!(WriteReplaceWarningResponse, non_ue_signalling);

// Implement for UnsuccessfulOutcome PDUs
impl_id_for_pdu!(AmfConfigurationUpdateFailure, non_ue_signalling);
impl_id_for_pdu!(BroadcastSessionModificationFailure, non_ue_signalling);
impl_id_for_pdu!(BroadcastSessionSetupFailure, non_ue_signalling);
impl_id_for_pdu!(DistributionSetupFailure, non_ue_signalling);
impl_id_for_pdu!(HandoverPreparationFailure, ue_signalling, amf_ran);
impl_id_for_pdu!(HandoverFailure, ue_signalling, amf);
impl_id_for_pdu!(InitialContextSetupFailure, ue_signalling, amf_ran);
impl_id_for_pdu!(MulticastSessionActivationFailure, non_ue_signalling);
impl_id_for_pdu!(MulticastSessionUpdateFailure, non_ue_signalling);
impl_id_for_pdu!(NgSetupFailure, non_ue_signalling);
impl_id_for_pdu!(PathSwitchRequestFailure, ue_signalling, amf_ran);
impl_id_for_pdu!(RanConfigurationUpdateFailure, non_ue_signalling);
impl_id_for_pdu!(UeContextModificationFailure, ue_signalling, amf_ran);
impl_id_for_pdu!(UeContextResumeFailure, ue_signalling, amf_ran);
impl_id_for_pdu!(UeContextSuspendFailure, ue_signalling, amf_ran);

impl Id<NgapSignallingId> for ErrorIndication {
	const MODE: NgapSignallingMode = NgapSignallingMode::Both;
	fn get_id(&self) -> NgapSignallingId {
        match (self.amf_ue_ngap_id, self.ran_ue_ngap_id) {
            (Some(amf_ue_ngap_id), Some(ran_ue_ngap_id)) => {
                NgapSignallingId::AmfRanUeNgapId(amf_ue_ngap_id, ran_ue_ngap_id)
            }
            (Some(amf_ue_ngap_id), None) => NgapSignallingId::AmfUeNgapId(amf_ue_ngap_id),
            (None, Some(ran_ue_ngap_id)) => NgapSignallingId::RanUeNgapId(ran_ue_ngap_id),
            // When None of the UE IDs are present, then the pdu is non-ue signalling
            (None, None) => NgapSignallingId::NonUeEmpty,
        }
	}
}

macro_rules! match_ue_pdu {
    ($msg:expr, $type:ident, $($variant:ident),*) => {
        match $msg {
            $(
                $type::$variant(pdu) => pdu.get_id(),
            )*
        }
    };
}

impl NgapPdu {
	pub fn get_signalling_id(&self) -> NgapSignallingId {
		match self {
			NgapPdu::InitiatingMessage(msg) => match_ue_pdu!(
				msg,
				InitiatingMessage,
				AmfConfigurationUpdate,
				AmfcpRelocationIndication,
				AmfStatusIndication,
				BroadcastSessionModificationRequest,
				BroadcastSessionReleaseRequest,
				BroadcastSessionReleaseRequired,
				BroadcastSessionSetupRequest,
				CellTrafficTrace,
				ConnectionEstablishmentIndication,
				DeactivateTrace,
				DistributionSetupRequest,
				DistributionReleaseRequest,
				DownlinkNasTransport,
				DownlinkNonUeAssociatedNrPPaTransport,
				DownlinkRanConfigurationTransfer,
				DownlinkRanEarlyStatusTransfer,
				DownlinkRanStatusTransfer,
				DownlinkUeAssociatedNrPPaTransport,
				ErrorIndication,
				HandoverCancel,
				HandoverNotify,
				HandoverRequired,
				HandoverRequest,
				HandoverSuccess,
				InitialContextSetupRequest,
				InitialUeMessage,
				LocationReport,
				LocationReportingControl,
				LocationReportingFailureIndication,
				MulticastSessionActivationRequest,
				MulticastSessionDeactivationRequest,
				MulticastSessionUpdateRequest,
				MulticastGroupPaging,
				NasNonDeliveryIndication,
				NgReset,
				NgSetupRequest,
				OverloadStart,
				OverloadStop,
				Paging,
				PathSwitchRequest,
				PduSessionResourceModifyRequest,
				PduSessionResourceModifyIndication,
				PduSessionResourceNotify,
				PduSessionResourceReleaseCommand,
				PduSessionResourceSetupRequest,
				PwsCancelRequest,
				PwsFailureIndication,
				PwsRestartIndication,
				RanConfigurationUpdate,
				RancpRelocationIndication,
				RerouteNasRequest,
				RetrieveUeInformation,
				RrcInactiveTransitionReport,
				SecondaryRatDataUsageReport,
				TraceFailureIndication,
				TraceStart,
				UeContextModificationRequest,
				UeContextReleaseCommand,
				UeContextReleaseRequest,
				UeContextResumeRequest,
				UeContextSuspendRequest,
				UeInformationTransfer,
				UeRadioCapabilityCheckRequest,
				UeRadioCapabilityIdMappingRequest,
				UeRadioCapabilityInfoIndication,
				UeTnlaBindingReleaseRequest,
				UplinkNasTransport,
				UplinkNonUeAssociatedNrPPaTransport,
				UplinkRanConfigurationTransfer,
				UplinkRanEarlyStatusTransfer,
				UplinkRanStatusTransfer,
				UplinkUeAssociatedNrPPaTransport,
				WriteReplaceWarningRequest,
				UplinkRimInformationTransfer,
				DownlinkRimInformationTransfer
			),
			NgapPdu::SuccessfulOutcome(msg) => match_ue_pdu!(
				msg,
				SuccessfulOutcome,
				AmfConfigurationUpdateAcknowledge,
				BroadcastSessionModificationResponse,
				BroadcastSessionReleaseResponse,
				BroadcastSessionSetupResponse,
				DistributionSetupResponse,
				DistributionReleaseResponse,
				HandoverCancelAcknowledge,
				HandoverCommand,
				HandoverRequestAcknowledge,
				InitialContextSetupResponse,
				MulticastSessionActivationResponse,
				MulticastSessionDeactivationResponse,
				MulticastSessionUpdateResponse,
				NgResetAcknowledge,
				NgSetupResponse,
				PathSwitchRequestAcknowledge,
				PduSessionResourceModifyResponse,
				PduSessionResourceModifyConfirm,
				PduSessionResourceReleaseResponse,
				PduSessionResourceSetupResponse,
				PwsCancelResponse,
				RanConfigurationUpdateAcknowledge,
				UeContextModificationResponse,
				UeContextReleaseComplete,
				UeContextResumeResponse,
				UeContextSuspendResponse,
				UeRadioCapabilityCheckResponse,
				UeRadioCapabilityIdMappingResponse,
				WriteReplaceWarningResponse
			),
			NgapPdu::UnsuccessfulOutcome(msg) => match_ue_pdu!(
				msg,
				UnsuccessfulOutcome,
				AmfConfigurationUpdateFailure,
				BroadcastSessionModificationFailure,
				BroadcastSessionSetupFailure,
				DistributionSetupFailure,
				HandoverPreparationFailure,
				HandoverFailure,
				InitialContextSetupFailure,
				MulticastSessionActivationFailure,
				MulticastSessionUpdateFailure,
				NgSetupFailure,
				PathSwitchRequestFailure,
				RanConfigurationUpdateFailure,
				UeContextModificationFailure,
				UeContextResumeFailure,
				UeContextSuspendFailure
			),
		}
	}
}
