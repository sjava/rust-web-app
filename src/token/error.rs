use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	HmacFailNewFromSlice,

	InvalidFormat,
	CannotDecodeIdent,
	CannotDecodeExp,
	SignatureNotMatching,
	ExpNotIso,
	Expired,
}

impl std::fmt::Display for Error {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "{self:?}")
	}
}
impl std::error::Error for Error {}
