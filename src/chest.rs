use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::compression::Compress;
use crate::crypto::{Encrypt, EncryptedPayload};

#[derive(Serialize, Deserialize)]
pub(crate) struct Chest {
    payload: EncryptedPayload,
}

impl Chest {
    pub(crate) fn create_from_payload(
        payload: &[u8],
        password: &str,
        compressor: &impl Compress,
        encryptor: &impl Encrypt,
    ) -> Result<Self> {
        let payload = compressor.compress(payload)?;
        let payload = encryptor.encrypt(payload, password)?;
        Ok(Self { payload })
    }

    pub(crate) fn create_from_file<P: AsRef<Path>>(
        path: P,
        password: &str,
        compressor: &impl Compress,
        encryptor: &impl Encrypt,
    ) -> Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut payload = Vec::new();
        file.read_to_end(&mut payload)?;
        Self::create_from_payload(&payload, password, compressor, encryptor)
    }

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
