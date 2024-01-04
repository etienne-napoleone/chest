use anyhow::Result;
use ring::aead;
use serde::{Deserialize, Serialize};

use crate::{
    chest::{EncryptedBlob, EncryptionAlgorithm},
    random::generate_random_bytes,
};

const SALT_LENGTH: usize = 8;
const NONCE_LENGTH: usize = 12;

pub(crate) fn get_encryptor(algorithm: &EncryptionAlgorithm) -> impl Encrypt {
    match algorithm {
        EncryptionAlgorithm::Aes256 => Aes256Encryptor,
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EncryptedPayload {
    pub(crate) cipher: Vec<u8>,
    pub(crate) salt: Vec<u8>,
    pub(crate) nonce: Vec<u8>,
}

pub(crate) trait Encrypt {
    fn encrypt(&self, payload: Vec<u8>, key: &[u8; 32]) -> Result<EncryptedBlob>;
    fn decrypt(&self, payload: &EncryptedBlob, key: &[u8; 32]) -> Result<Vec<u8>>;
}

#[derive(Default)]
pub(crate) struct Aes256Encryptor;

impl Encrypt for Aes256Encryptor {
    fn encrypt(&self, payload: Vec<u8>, key: &[u8; 32]) -> Result<EncryptedBlob> {
        let mut buffer = payload.clone();
        // salt
        let salt = generate_random_bytes(SALT_LENGTH)?;
        let aead_alg = &aead::AES_256_GCM;
        let sealing_key = aead::LessSafeKey::new(aead::UnboundKey::new(aead_alg, key).unwrap());
        // nonce
        let raw_nonce = generate_random_bytes(NONCE_LENGTH)?;
        let nonce = aead::Nonce::assume_unique_for_key(raw_nonce.clone().try_into().unwrap());
        // encrypt
        sealing_key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut buffer)?;
        Ok(EncryptedBlob {
            cipher: buffer,
            salt,
            nonce: raw_nonce,
        })
    }

    fn decrypt(&self, payload: &EncryptedBlob, key: &[u8; 32]) -> Result<Vec<u8>> {
        let mut buffer = payload.cipher.clone();
        let aead_alg = &aead::AES_256_GCM;
        let sealing_key = aead::LessSafeKey::new(aead::UnboundKey::new(aead_alg, key).unwrap());
        // nonce
        let nonce = aead::Nonce::assume_unique_for_key(payload.nonce.clone().try_into().unwrap());
        // decrypt
        let plaintext = sealing_key.open_in_place(nonce, aead::Aad::empty(), &mut buffer)?;
        Ok(plaintext.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &[u8; 32] = &[
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1,
    ];
    const ENCRYPTOR: Aes256Encryptor = Aes256Encryptor;
    const PAYLOAD: &[u8; 9] = b"some data";

    #[test]
    fn encrypted_then_decrypted_should_match() {
        let encrypted = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), KEY).unwrap();
        let decrypted = ENCRYPTOR.decrypt(&encrypted, KEY).unwrap();
        assert_eq!(PAYLOAD, decrypted.as_slice());
    }

    #[test]
    fn encrypted_should_be_different_than_decrypted() {
        let encrypted = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), KEY).unwrap();
        assert_ne!(PAYLOAD, encrypted.cipher.as_slice());
    }

    #[test]
    fn encrypted_should_be_different_with_different_key() {
        let encrypted_one = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), KEY).unwrap();
        let encrypted_two = ENCRYPTOR
            .encrypt(
                PAYLOAD.to_vec(),
                &[
                    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
                    2, 2, 2, 2, 2, 2,
                ],
            )
            .unwrap();
        assert_ne!(encrypted_one.cipher, encrypted_two.cipher);
    }

    #[test]
    fn salt_should_be_random() {
        let encrypted_one = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), KEY).unwrap();
        let encrypted_two = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), KEY).unwrap();
        assert_ne!(encrypted_one.salt, encrypted_two.salt);
    }

    #[test]
    fn nonce_should_be_random() {
        let encrypted_one = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), KEY).unwrap();
        let encrypted_two = ENCRYPTOR.encrypt(PAYLOAD.to_vec(), KEY).unwrap();
        assert_ne!(encrypted_one.nonce, encrypted_two.nonce);
    }
}
