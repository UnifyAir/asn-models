use thiserror::Error;
use crate::PerCodecError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Error, Debug)]
pub struct Error {
	#[source]
	pub codec_error: PerCodecError,
	pub diagnostics: Vec<Diagnostic>,
}

impl Error {
	pub fn new<T: AsRef<str> + Display>(msg: T) -> Self {
        Error {
            codec_error: PerCodecError::new(msg.to_string()),
            diagnostics: Vec::new(),
        }
    }
}

impl From<PerCodecError> for Error {
	fn from(codec_error: PerCodecError) -> Self {
		Self { codec_error, diagnostics: vec![] }
	}
}

impl Display for Error {
	fn fmt(
		&self,
		f: &mut Formatter<'_>,
	) -> FmtResult {
		write!(f, "{}", self.codec_error)
	}
}

#[derive(Clone, Debug)]
pub struct Diagnostic {
	pub ie_criticality: Criticality,
	pub ie_id: u16,
	pub type_of_error: TypeOfError,
}

#[derive(Clone, Debug)]
pub enum Criticality {
	Reject,
	Ignore,
	Notify,
}

#[derive(Clone, Debug)]
pub enum TypeOfError {
	NotUnderstood,
	Missing,
}