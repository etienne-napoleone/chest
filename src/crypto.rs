use std::num::NonZeroU32;

use anyhow::Result;
use ring::rand::{SecureRandom, SystemRandom};
use ring::{aead, pbkdf2};
use serde::{Deserialize, Serialize};

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;

pub(crate) trait Encrypt {
    fn encrypt(&self, payload: Vec<u8>, password: &str) -> Result<EncryptedPayload>;
    fn decrypt(&self, payload: &EncryptedPayload, password: &str) -> Result<Vec<u8>>;
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EncryptedPayload {
    pub(crate) cipher: Vec<u8>,
    pub(crate) salt: [u8; 8],
    pub(crate) nonce: [u8; 12],
}

#[derive(Default)]
pub(crate) struct Aes256Encryptor;

impl Encrypt for Aes256Encryptor {
    fn encrypt(&self, payload: Vec<u8>, password: &str) -> Result<EncryptedPayload> {
        let mut buffer = payload.clone();
        let rng = SystemRandom::new();
        // salt
        let mut salt = [0u8; 8];
        rng.fill(&mut salt)?;
        // key
        let mut key = [0u8; 32];
        pbkdf2::derive(
            PBKDF2_ALG,
            NonZeroU32::new(100_000).expect("Could not generate nonce"),
            &salt,
            password.as_bytes(),
            &mut key,
        );
        let aead_alg = &aead::AES_256_GCM;
        let sealing_key = aead::LessSafeKey::new(aead::UnboundKey::new(aead_alg, &key).unwrap());
        // nonce
        let mut raw_nonce = [0; 12];
        rng.fill(&mut raw_nonce)?;
        let nonce = aead::Nonce::assume_unique_for_key(raw_nonce);
        // encrypt
        sealing_key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut buffer)?;
        Ok(EncryptedPayload {
            cipher: buffer,
            salt,
            nonce: raw_nonce,
        })
    }

    fn decrypt(&self, payload: &EncryptedPayload, password: &str) -> Result<Vec<u8>> {
        let mut buffer = payload.cipher.clone();
        // key
        let mut key = [0u8; 32];
        pbkdf2::derive(
            PBKDF2_ALG,
            NonZeroU32::new(100_000).expect("Could not generate nonce"),
            &payload.salt,
            password.as_bytes(),
            &mut key,
        );
        let aead_alg = &aead::AES_256_GCM;
        let sealing_key = aead::LessSafeKey::new(aead::UnboundKey::new(aead_alg, &key).unwrap());
        // nonce
        let nonce = aead::Nonce::assume_unique_for_key(payload.nonce);
        // decrypt
        let plaintext = sealing_key.open_in_place(nonce, aead::Aad::empty(), &mut buffer)?;
        Ok(plaintext.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PASSWORD: &str = "this is a password";
    const ENCRYPTOR: Aes256Encryptor = Aes256Encryptor;
    const PAYLOAD: &[u8; 9] = b"some data";

    #[test]
    fn encrypted_then_decrypted_should_match() {
        let encrypted = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), PASSWORD).unwrap();
        let decrypted = ENCRYPTOR.decrypt(&encrypted, PASSWORD).unwrap();
        assert_eq!(PAYLOAD, decrypted.as_slice());
    }

    #[test]
    fn encrypted_should_be_different_than_decrypted() {
        let encrypted = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), PASSWORD).unwrap();
        assert_ne!(PAYLOAD, encrypted.cipher.as_slice());
    }

    #[test]
    fn encrypted_should_be_different_with_different_password() {
        let encrypted_one = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), PASSWORD).unwrap();
        let encrypted_two = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), "abcd").unwrap();
        assert_ne!(encrypted_one.cipher, encrypted_two.cipher);
    }

    #[test]
    fn salt_should_be_random() {
        let encrypted_one = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), PASSWORD).unwrap();
        let encrypted_two = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), PASSWORD).unwrap();
        assert_ne!(encrypted_one.salt, encrypted_two.salt);
    }

    #[test]
    fn nonce_should_be_random() {
        let encrypted_one = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), PASSWORD).unwrap();
        let encrypted_two = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), PASSWORD).unwrap();
        assert_ne!(encrypted_one.nonce, encrypted_two.nonce);
    }
}
