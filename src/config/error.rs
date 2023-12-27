pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	MissingEnv(&'static str),
	WrongFormat(&'static str),
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
