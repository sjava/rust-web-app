use base64::engine::{general_purpose, Engine};

pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
	general_purpose::URL_SAFE_NO_PAD.encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>> {
	general_purpose::URL_SAFE_NO_PAD
		.decode(b64u)
		.map_err(|_| Error::FailToB64uDecode)
}

pub fn b64u_decode_to_string(b64u: &str) -> Result<String> {
	b64u_decode(b64u)
		.ok()
		.and_then(|bytes| String::from_utf8(bytes).ok())
		.ok_or(Error::FailToB64uDecode)
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
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
