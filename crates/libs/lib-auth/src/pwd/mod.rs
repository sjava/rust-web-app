mod error;
mod hmac_hasher;

pub use self::error::{Error, Result};

use crate::auth_config;
use crate::pwd::hmac_hasher::hmac_sha512_hash;
use uuid::Uuid;

pub struct ContentToHash {
	pub content: String,
	pub salt: Uuid,
}

pub fn hash_pwd(to_hash: &ContentToHash) -> Result<String> {
	let key = &auth_config().PWD_KEY;
	let hashed = hmac_sha512_hash(key, to_hash)?;
	Ok(format!("#01#{hashed}"))
}

pub fn validate_pwd(enc_content: &ContentToHash, pwd_ref: &str) -> Result<()> {
	let pwd = hash_pwd(enc_content)?;
	if pwd_ref == &pwd {
		Ok(())
	} else {
		Err(Error::NotMatching)
	}
}
