use std::num::NonZeroU32;

use anyhow::Result;
use ring::pbkdf2;

static PBKDF2_ALGORITHM: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

const PBKDF2_LENGTH: usize = 32;

pub(crate) trait Derive {
    fn derive(&self, password: &str, salt: Vec<u8>) -> Result<Vec<u8>>;
}

pub(crate) struct Pbkdf2HmacSha256Deriver;

impl Derive for Pbkdf2HmacSha256Deriver {
    fn derive(&self, password: &str, salt: Vec<u8>) -> Result<Vec<u8>> {
        let mut key = [0u8; PBKDF2_LENGTH];
        pbkdf2::derive(
            PBKDF2_ALGORITHM,
            NonZeroU32::new(100_000).expect("Could not generate iteration"),
            &salt,
            password.as_bytes(),
            &mut key,
        );
        Ok(key.to_vec())
    }
}
