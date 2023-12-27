use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime};

pub fn now_utc() -> OffsetDateTime {
	OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
	time.format(&Rfc3339).unwrap()
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
	let now = now_utc();
	let now_plus_sec = now + Duration::seconds_f64(sec);
	format_time(now_plus_sec)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
	OffsetDateTime::parse(moment, &Rfc3339)
		.map_err(|_| Error::FailToDateParse(moment.to_owned()))
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	FailToDateParse(String),
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
