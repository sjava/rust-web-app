pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// -- Time
	DateFailParse(String),

	// -- Base64
	FailToB64uDecode,
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
