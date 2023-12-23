mod error;

pub use self::error::{Error, Result};

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
		.map_err(|_| Error::DateFailParse(moment.to_owned()))
}

pub fn b64u_encode(content: &str) -> String {
	base64_url::encode(content)
}

pub fn b64u_decode(content: &str) -> Result<String> {
	base64_url::decode(content)
		.ok()
		.and_then(|bytes| String::from_utf8(bytes).ok())
		.ok_or(Error::FailToB64uDecode)
}
