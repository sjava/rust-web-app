use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	// -- Key
	KeyFailHmac,

	// -- Pwd
	PwdNotMatching,

	// --Token
	TokenInvalidFormat,
	TokenCannotDecodeIdent,
	TokenCannotDecodeExp,
	TokenSignatureNotMatching,
	TokenExpNotIso,
	TokenExpired,
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
