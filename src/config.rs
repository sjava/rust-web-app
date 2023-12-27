use crate::{Error, Result};
use crate::utils::b64u_decode;
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
	static INSTANCE: OnceLock<Config> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		Config::load_from_env()
			.unwrap_or_else(|e| panic!("FATAL -	WHILE LOADING CONF - Cause: {e:?}"))
	})
}

#[allow(non_snake_case)]
pub struct Config {
	// -- Keys
	pub PWD_KEY: Vec<u8>,

	pub TOKEN_KEY: Vec<u8>,
	pub TOKEN_DURATION_SEC: f64,

	// -- DB
	pub DB_URL: String,
	pub WEB_FOLDER: String,
}

impl Config {
	fn load_from_env() -> Result<Config> {
		Ok(Config {
			PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,
			TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
			TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,
			DB_URL: get_env("SERVICE_DB_URL")?,
			WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
		})
	}
}

fn get_env(name: &'static str) -> Result<String> {
	env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_parse<T>(name: &'static str) -> Result<T>
where
	T: FromStr,
{
	get_env(name)?
		.parse::<T>()
		.map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
	b64u_decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}
