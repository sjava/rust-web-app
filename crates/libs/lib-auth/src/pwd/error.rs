use crate::pwd::scheme;
use derive_more::From;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize,From)]
pub enum Error {
	// -- Key
	PwdWithSchemeFailedParse,

	// -- Modules
	#[from]
	Scheme(scheme::Error),
}

impl std::fmt::Display for Error {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter,
	) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{self:?}")
	}
}

impl std::error::Error for Error {}
