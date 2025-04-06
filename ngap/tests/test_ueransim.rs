use asn1_per::SerDes;
use ngap::{NgapPdu, PduSessionResourceSetupRequestTransfer};

pub fn get_ng_setup_request() -> Vec<u8> {
	vec![
		0x0, 0x15, 0x0, 0x35, 0x0, 0x0, 0x4, 0x0, 0x1b, 0x0, 0x8, 0x0, 0x2, 0xf8, 0x39, 0x10, 0x0,
		0x1, 0x2, 0x0, 0x52, 0x40, 0x9, 0x3, 0x0, 0x66, 0x72, 0x65, 0x65, 0x35, 0x67, 0x63, 0x0,
		0x66, 0x0, 0x10, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x2, 0xf8, 0x39, 0x0, 0x0, 0x10, 0x8, 0x1,
		0x2, 0x3, 0x0, 0x15, 0x40, 0x1, 0x40,
	]
}

pub fn get_ng_setup_response() -> Vec<u8> {
	vec![
		0x20, 0x15, 0x0, 0x31, 0x0, 0x0, 0x4, 0x0, 0x1, 0x0, 0x5, 0x1, 0x0, 0x41, 0x4d, 0x46, 0x0,
		0x60, 0x0, 0x8, 0x0, 0x0, 0x2, 0xf8, 0x39, 0xca, 0xfe, 0x0, 0x0, 0x56, 0x40, 0x1, 0xff,
		0x0, 0x50, 0x0, 0x10, 0x0, 0x2, 0xf8, 0x39, 0x0, 0x1, 0x10, 0x8, 0x1, 0x2, 0x3, 0x10, 0x8,
		0x11, 0x22, 0x33,
	]
}

pub fn get_initial_ue_message() -> Vec<u8> {
	vec![
		0x0, 0xf, 0x40, 0x41, 0x0, 0x0, 0x5, 0x0, 0x55, 0x0, 0x2, 0x0, 0x1, 0x0, 0x26, 0x0, 0x17,
		0x16, 0x7e, 0x0, 0x41, 0x79, 0x0, 0xc, 0x1, 0x2, 0xf8, 0x39, 0xf0, 0xff, 0x0, 0x0, 0x0,
		0x0, 0x47, 0x78, 0x2e, 0x2, 0x80, 0x20, 0x0, 0x79, 0x0, 0xf, 0x40, 0x2, 0xf8, 0x39, 0x0,
		0x0, 0x0, 0x0, 0x10, 0x2, 0xf8, 0x39, 0x0, 0x0, 0x1, 0x0, 0x5a, 0x40, 0x1, 0x10, 0x0, 0x70,
		0x40, 0x1, 0x0,
	]
}

pub fn get_downlink_nas_transport_authentication_req() -> Vec<u8> {
	vec![
		0x0, 0x4, 0x40, 0x3e, 0x0, 0x0, 0x3, 0x0, 0xa, 0x0, 0x2, 0x0, 0x1, 0x0, 0x55, 0x0, 0x2,
		0x0, 0x1, 0x0, 0x26, 0x0, 0x2b, 0x2a, 0x7e, 0x0, 0x56, 0x0, 0x2, 0x0, 0x0, 0x21, 0xa1,
		0xbd, 0xd3, 0xc9, 0xed, 0x37, 0xd6, 0x0, 0xba, 0x8b, 0x76, 0xe, 0xdf, 0x82, 0x1a, 0x5f,
		0x20, 0x10, 0xff, 0x2d, 0x27, 0x7, 0xe8, 0xeb, 0x80, 0x0, 0x67, 0x1c, 0x47, 0x7e, 0x5f,
		0x60, 0x62, 0xb6,
	]
}

pub fn get_uplink_nas_transport_authentication_response() -> Vec<u8> {
	vec![
		0x0, 0x2e, 0x40, 0x3c, 0x0, 0x0, 0x4, 0x0, 0xa, 0x0, 0x2, 0x0, 0x1, 0x0, 0x55, 0x0, 0x2,
		0x0, 0x1, 0x0, 0x26, 0x0, 0x16, 0x15, 0x7e, 0x0, 0x57, 0x2d, 0x10, 0x1e, 0x76, 0x6c, 0x11,
		0xd3, 0x31, 0x40, 0xc0, 0x15, 0xcc, 0xf4, 0x5c, 0x4, 0xea, 0x71, 0x13, 0x0, 0x79, 0x40,
		0xf, 0x40, 0x2, 0xf8, 0x39, 0x0, 0x0, 0x0, 0x0, 0x10, 0x2, 0xf8, 0x39, 0x0, 0x0, 0x1,
	]
}

pub fn get_downlin_nas_transport_security_mod_command() -> Vec<u8> {
	vec![
		0x0, 0x4, 0x40, 0x27, 0x0, 0x0, 0x3, 0x0, 0xa, 0x0, 0x2, 0x0, 0x1, 0x0, 0x55, 0x0, 0x2,
		0x0, 0x1, 0x0, 0x26, 0x0, 0x14, 0x13, 0x7e, 0x3, 0xd4, 0xb9, 0x61, 0x3, 0x0, 0x7e, 0x0,
		0x5d, 0x2, 0x0, 0x2, 0x80, 0x20, 0xe1, 0x36, 0x1, 0x0,
	]
}

pub fn get_uplink_nas_ssc_mode_registration_req() -> Vec<u8> {
	vec![
		0x0, 0x2e, 0x40, 0x59, 0x0, 0x0, 0x4, 0x0, 0xa, 0x0, 0x2, 0x0, 0x1, 0x0, 0x55, 0x0, 0x2,
		0x0, 0x1, 0x0, 0x26, 0x0, 0x33, 0x32, 0x7e, 0x4, 0x4a, 0xe9, 0x72, 0x1f, 0x0, 0x7e, 0x0,
		0x5e, 0x77, 0x0, 0x9, 0x15, 0x11, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x71, 0x0, 0x19, 0x7e,
		0x0, 0x41, 0x79, 0x0, 0xc, 0x1, 0x2, 0xf8, 0x39, 0xf0, 0xff, 0x0, 0x0, 0x0, 0x0, 0x47,
		0x78, 0x10, 0x1, 0x7, 0x2e, 0x2, 0x80, 0x20, 0x0, 0x79, 0x40, 0xf, 0x40, 0x2, 0xf8, 0x39,
		0x0, 0x0, 0x0, 0x0, 0x10, 0x2, 0xf8, 0x39, 0x0, 0x0, 0x1,
	]
}

pub fn get_initial_ctx_setup_req_registration_accept() -> Vec<u8> {
	vec![
		0x0, 0xe, 0x0, 0x80, 0xa7, 0x0, 0x0, 0x9, 0x0, 0xa, 0x0, 0x2, 0x0, 0x1, 0x0, 0x55, 0x0,
		0x2, 0x0, 0x1, 0x0, 0x1c, 0x0, 0x7, 0x0, 0x2, 0xf8, 0x39, 0xca, 0xfe, 0x0, 0x0, 0x0, 0x0,
		0xa, 0x22, 0x1, 0x1, 0x2, 0x3, 0x10, 0x8, 0x11, 0x22, 0x33, 0x0, 0x77, 0x0, 0x9, 0x0, 0x0,
		0x0, 0x10, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5e, 0x0, 0x20, 0xdb, 0x10, 0x82, 0x1b, 0x1e,
		0xb2, 0xc, 0x3b, 0xb6, 0x51, 0x27, 0x65, 0xd9, 0xd2, 0xa7, 0xbd, 0x8e, 0x9f, 0xe6, 0x1b,
		0x39, 0xbf, 0xb1, 0x90, 0x25, 0xe0, 0xf7, 0xb9, 0x82, 0x7b, 0xa5, 0x4c, 0x0, 0x24, 0x40,
		0x4, 0x0, 0x2, 0xf8, 0x39, 0x0, 0x22, 0x40, 0x8, 0x11, 0x10, 0x0, 0x0, 0x0, 0xff, 0xff,
		0x0, 0x0, 0x26, 0x40, 0x36, 0x35, 0x7e, 0x2, 0x34, 0xb6, 0x53, 0x4b, 0x1, 0x7e, 0x0, 0x42,
		0x1, 0x1, 0x77, 0x0, 0xb, 0x2, 0x2, 0xf8, 0x39, 0xca, 0xfe, 0x0, 0x0, 0x0, 0x0, 0x1, 0x54,
		0x7, 0x0, 0x2, 0xf8, 0x39, 0x0, 0x0, 0x1, 0x15, 0xa, 0x4, 0x1, 0x1, 0x2, 0x3, 0x4, 0x1,
		0x11, 0x22, 0x33, 0x5e, 0x1, 0x6, 0x16, 0x1, 0x2c,
	]
}

pub fn initial_ctx_setup_response() -> Vec<u8> {
	vec![
		0x20, 0xe, 0x0, 0xf, 0x0, 0x0, 0x2, 0x0, 0xa, 0x40, 0x2, 0x0, 0x1, 0x0, 0x55, 0x40, 0x2,
		0x0, 0x1,
	]
}

pub fn get_uplink_nas_transport_registration_complete() -> Vec<u8> {
	vec![
		0x0, 0x2e, 0x40, 0x5c, 0x0, 0x0, 0x4, 0x0, 0xa, 0x0, 0x2, 0x0, 0x1, 0x0, 0x55, 0x0, 0x2,
		0x0, 0x1, 0x0, 0x26, 0x0, 0x36, 0x35, 0x7e, 0x2, 0xf3, 0x3b, 0xed, 0xbf, 0x2, 0x7e, 0x0,
		0x67, 0x1, 0x0, 0x14, 0x2e, 0xa, 0x0, 0xc1, 0xff, 0xff, 0x91, 0x7b, 0x0, 0xa, 0x80, 0x0,
		0xa, 0x0, 0x0, 0xd, 0x0, 0x0, 0x3, 0x0, 0x12, 0xa, 0x81, 0x22, 0x4, 0x1, 0x1, 0x2, 0x3,
		0x25, 0x9, 0x8, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x65, 0x74, 0x0, 0x79, 0x40, 0xf, 0x40,
		0x2, 0xf8, 0x39, 0x0, 0x0, 0x0, 0x0, 0x10, 0x2, 0xf8, 0x39, 0x0, 0x0, 0x1,
	]
}

pub fn get_pdu_session_resource_setup_request() -> Vec<u8> {
	vec![
		0x0, 0x1d, 0x0, 0x80, 0xa6, 0x0, 0x0, 0x3, 0x0, 0xa, 0x0, 0x2, 0x0, 0x1, 0x0, 0x55, 0x0,
		0x2, 0x0, 0x1, 0x0, 0x4a, 0x0, 0x80, 0x92, 0x0, 0x40, 0xa, 0x67, 0x7e, 0x2, 0xbe, 0x15,
		0xe1, 0x45, 0x2, 0x7e, 0x0, 0x68, 0x1, 0x0, 0x58, 0x2e, 0xa, 0x0, 0xc2, 0x11, 0x0, 0x9,
		0x1, 0x0, 0x6, 0x31, 0x31, 0x1, 0x1, 0xff, 0x9, 0x6, 0xb, 0x0, 0x1, 0xb, 0x0, 0x2, 0x59,
		0x32, 0x29, 0x5, 0x1, 0x3c, 0x3c, 0x0, 0x1, 0x22, 0x4, 0x1, 0x1, 0x2, 0x3, 0x79, 0x0, 0x6,
		0x9, 0x20, 0x41, 0x1, 0x1, 0x9, 0x7b, 0x0, 0x1b, 0x80, 0x0, 0xd, 0x4, 0x8, 0x8, 0x8, 0x8,
		0x0, 0x3, 0x10, 0x20, 0x1, 0x48, 0x60, 0x48, 0x60, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
		0x88, 0x88, 0x25, 0x9, 0x8, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x65, 0x74, 0x12, 0xa,
		0x40, 0x20, 0x1, 0x2, 0x3, 0x21, 0x0, 0x0, 0x3, 0x0, 0x8b, 0x0, 0xa, 0x1, 0xf0, 0xa, 0x64,
		0xc8, 0x3, 0x0, 0x0, 0x0, 0x1, 0x0, 0x86, 0x0, 0x1, 0x0, 0x0, 0x88, 0x0, 0x7, 0x0, 0x0,
		0x0, 0x0, 0x9, 0x38, 0x0,
	]
}

pub fn get_pdu_session_resource_setup_response() -> Vec<u8> {
	vec![
		0x20, 0x1d, 0x0, 0x24, 0x0, 0x0, 0x3, 0x0, 0xa, 0x40, 0x2, 0x0, 0x1, 0x0, 0x55, 0x40, 0x2,
		0x0, 0x1, 0x0, 0x4b, 0x40, 0x11, 0x0, 0x0, 0xa, 0xd, 0x0, 0x3, 0xe0, 0xa, 0x64, 0xc8, 0x1,
		0x0, 0x0, 0x0, 0x1, 0x0, 0x1,
	]
}


pub fn get_pdu_session_resource_setup_request_transfer_element() -> Vec<u8> {
	vec![
		0x0, 0x0, 0x3, 0x0, 0x8b, 0x0, 0xa, 0x1, 0xf0, 0xa, 0x64, 0xc8, 0x3, 0x0, 0x0, 0x0, 0x1,
		0x0, 0x86, 0x0, 0x1, 0x0, 0x0, 0x88, 0x0, 0x7, 0x0, 0x0, 0x0, 0x0, 0x9, 0x38, 0x0,
	]
}

pub fn encode_decode_bytes<T>(payload: &[u8]) -> Vec<u8>
where
	T: SerDes + std::fmt::Debug,
{
	let decoded_req: T = T::from_bytes(payload).unwrap();
	println!("decoded_req {:#?}", decoded_req);
	T::into_bytes(decoded_req).unwrap()
}

#[test]
fn test_ng_setup_request() {
	let payload = get_ng_setup_request();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_ng_setup_response() {
	let payload = get_ng_setup_response();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_initial_ue_message() {
	let payload = get_initial_ue_message();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_downlink_nas_transport_authentication_req() {
	let payload = get_downlink_nas_transport_authentication_req();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_uplink_nas_transport_authentication_response() {
	let payload = get_uplink_nas_transport_authentication_response();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_downlin_nas_transport_security_mod_command() {
	let payload = get_downlin_nas_transport_security_mod_command();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_uplink_nas_ssc_mode_registration_req() {
	let payload = get_uplink_nas_ssc_mode_registration_req();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_initial_ctx_setup_req_registration_accept() {
	let payload = get_initial_ctx_setup_req_registration_accept();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_initial_ctx_setup_response() {
	let payload = initial_ctx_setup_response();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_uplink_nas_transport_registration_complete() {
	let payload = get_uplink_nas_transport_registration_complete();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_pdu_session_resource_setup_request() {
	let payload = get_pdu_session_resource_setup_request();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_pdu_session_resource_setup_response() {
	let payload = get_pdu_session_resource_setup_response();
	let bytes = encode_decode_bytes::<NgapPdu>(payload.as_ref());
	assert_eq!(payload, bytes);
}

#[test]
fn test_pdu_session_resource_setup_request_transfer_element() {
	let payload = get_pdu_session_resource_setup_request_transfer_element();
	let bytes = encode_decode_bytes::<PduSessionResourceSetupRequestTransfer>(payload.as_ref());
	assert_eq!(payload, bytes);
}

