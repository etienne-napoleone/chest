use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::Result;
// use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};

// use crate::compression::Compress;
// use crate::crypto::Encrypt;

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Chest {
    pub(crate) header: Header,
    pub(crate) files: Vec<EncryptedFile>,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Header {
    pub(crate) compression_algorithm: Option<CompressionAlgorithm>,
    pub(crate) key_derivation_algorithm: KeyDerivationAlgorithm,
    pub(crate) encryption_algorithm: EncryptionAlgorithm,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) enum CompressionAlgorithm {
    #[default]
    Deflate,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) enum EncryptionAlgorithm {
    #[default]
    Aes256,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) enum KeyDerivationAlgorithm {
    #[default]
    Pbkdf2HmacSha256,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EncryptedFile {
    pub(crate) metadata: Metadata,
    pub(crate) cipher: Vec<u8>,
    pub(crate) salt: [u8; 8],
    pub(crate) nonce: [u8; 12],
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Metadata {
    pub(crate) filename: String,
    pub(crate) size_bytes: u64,
}

impl Chest {
    // pub(crate) fn create_from_payload(
    //     payload: &[u8],
    //     password: &str,
    //     metadata: Metadata,
    //     compressor: &impl Compress,
    //     encryptor: &impl Encrypt,
    // ) -> Result<Self> {
    //     let payload = compressor.compress(payload)?;
    //     let payload = encryptor.encrypt(payload, password)?;

    //     Ok(Self {
    //         compressed: true,
    //         files: vec![EncryptedFile {
    //             metadata,
    //             cipher: payload.cipher,
    //             salt: payload.salt,
    //             nonce: payload.nonce,
    //         }],
    //     })
    // }

    // pub(crate) fn create_from_file<P: AsRef<Path>>(
    //     path: P,
    //     password: &str,
    //     compressor: &impl Compress,
    //     encryptor: &impl Encrypt,
    // ) -> Result<Self> {
    //     let mut file = File::open(&path)?;
    //     let metadata = File::metadata(&file)?;
    //     let metadata = Metadata {
    //         filename: path
    //             .as_ref()
    //             .file_name()
    //             .unwrap_or_default()
    //             .to_string_lossy()
    //             .into_owned(),
    //         size_bytes: metadata.len(),
    //     };
    //     let mut payload = Vec::new();
    //     file.read_to_end(&mut payload)?;
    //     Self::create_from_payload(&payload, password, metadata, compressor, encryptor)
    // }

    pub(crate) fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let serialized = bincode::serialize(self)?;
        let mut file = File::create(path)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    pub(crate) fn open_from_payload(payload: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(payload)?)
    }

    pub(crate) fn open_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut payload = Vec::new();
        file.read_to_end(&mut payload)?;
        Self::open_from_payload(&payload)
    }
}
