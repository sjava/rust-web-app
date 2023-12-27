use super::{ContentToHash, Error, Result};
use crate::utils::b64::b64u_encode;
use hmac::{Hmac, Mac};
use sha2::Sha512;

pub fn hmac_sha512_hash(key: &[u8], to_hash: &ContentToHash) -> Result<String> {
	let ContentToHash { content, salt } = to_hash;

	let mut hmac_sha512 =
		Hmac::<Sha512>::new_from_slice(key).map_err(|_| Error::KeyFail)?;
	hmac_sha512.update(content.as_bytes());
	hmac_sha512.update(salt.as_bytes());

	let hmac_result= hmac_sha512.finalize();
	let result=b64u_encode(hmac_result.into_bytes());
    Ok(result)
}
