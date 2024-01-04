use std::num::NonZeroU32;

use anyhow::Result;
use ring::pbkdf2;

static PBKDF2_ALGORITHM: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

pub(crate) trait Derive {
    fn derive(&self, password: &str, salt: [u8; 8]) -> Result<[u8; 32]>;
}

pub(crate) struct Pbkdf2HmacSha256Deriver;

impl Derive for Pbkdf2HmacSha256Deriver {
    fn derive(&self, password: &str, salt: [u8; 8]) -> Result<[u8; 32]> {
        let mut key = [0u8; 32];
        pbkdf2::derive(
            PBKDF2_ALGORITHM,
            NonZeroU32::new(100_000).expect("Could not generate iteration"),
            &salt,
            password.as_bytes(),
            &mut key,
        );
        Ok(key)
    }
}
