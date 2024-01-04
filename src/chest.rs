use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::compression::{get_compressor, Compress};
use crate::crypto::{get_encryptor, Encrypt};
use crate::key::{get_deriver, Derive};

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) enum CompressionAlgorithm {
    #[default]
    Deflate,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) enum EncryptionAlgorithm {
    #[default]
    Aes256,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) enum KeyDerivationAlgorithm {
    #[default]
    Pbkdf2HmacSha256,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct LockedHeader(EncryptedBlob);

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct UnlockedHeader {
    key: Vec<u8>,
}

pub(crate) trait Header {}
impl Header for LockedHeader {}
impl Header for UnlockedHeader {}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct Chest<H = UnlockedHeader>
where
    H: Header,
{
    pub(crate) public: Public,
    pub(crate) header: H,
    pub(crate) files: Vec<File>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct File {
    pub(crate) encrypted_blob: EncryptedBlob,
    pub(crate) metadata: Metadata,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct EncryptedBlob {
    pub(crate) cipher: Vec<u8>,
    pub(crate) salt: Vec<u8>,
    pub(crate) nonce: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Metadata {
    pub(crate) filename: String,
    pub(crate) size_bytes: u64,
}

impl Chest {
    pub(crate) fn new(password: &str) -> Result<Self> {
        let public = Public::default();
        let deriver = get_deriver(&public.key_derivation_algorithm);
        let key = deriver.derive(password, &public.key_derivation_salt)?;
        Ok(Self {
            public: Public::default(),
            header: UnlockedHeader { key },
            files: Vec::default(),
        })
    }
}

impl Chest<UnlockedHeader> {
    pub(crate) fn add_file_from_payload(
        &mut self,
        payload: Vec<u8>,
        metadata: Metadata,
    ) -> Result<()> {
        let encryptor = get_encryptor(&self.public.encryption_algorithm);
        let payload = match &self.public.compression_algorithm {
            Some(compression_algorithm) => {
                let compressor = get_compressor(compression_algorithm);
                compressor.compress(&payload)?
            }
            None => payload,
        };
        let file = File {
            encrypted_blob: encryptor
                .encrypt(payload, &self.header.clone().key.try_into().unwrap())?,
            metadata,
        };
        self.files.push(file);
        Ok(())
    }

    pub(crate) fn add_file_from_path<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let mut file = fs::File::open(&path)?;
        let metadata = fs::File::metadata(&file)?;
        let metadata = Metadata {
            filename: path
                .as_ref()
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            size_bytes: metadata.len(),
        };
        let mut payload = Vec::new();
        file.read_to_end(&mut payload)?;
        self.add_file_from_payload(payload, metadata)?;
        Ok(())
    }

    pub(crate) fn lock(&self, password: &str) -> Result<Chest<LockedHeader>> {
        let deriver = get_deriver(&self.public.key_derivation_algorithm);
        let encryptor = get_encryptor(&self.public.encryption_algorithm);
        let key = deriver.derive(password, &self.public.key_derivation_salt)?;
        let header = LockedHeader(
            encryptor.encrypt(bincode::serialize(&self.header)?, &key.try_into().unwrap())?,
        );
        Ok(Chest {
            header,
            public: self.public.clone(),
            files: self.files.clone(),
        })
    }
}

impl Chest<LockedHeader> {
    pub(crate) fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let serialized = bincode::serialize(self)?;
        let mut file = fs::File::create(path)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    pub(crate) fn unlock(&self, password: &str) -> Result<Chest<UnlockedHeader>> {
        let deriver = get_deriver(&self.public.key_derivation_algorithm);
        let encryptor = get_encryptor(&self.public.encryption_algorithm);
        let key = deriver.derive(password, &self.public.key_derivation_salt)?;
        let header = bincode::deserialize::<UnlockedHeader>(
            &encryptor.decrypt(&self.header.0, &key.try_into().unwrap())?,
        )?;
        Ok(Chest {
            header,
            public: self.public.clone(),
            files: self.files.clone(),
        })
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct Public {
    compression_algorithm: Option<CompressionAlgorithm>,
    key_derivation_algorithm: KeyDerivationAlgorithm,
    key_derivation_salt: Vec<u8>,
    encryption_algorithm: EncryptionAlgorithm,
}

// pub(crate) struct LockedChest {
//     pub(crate) header: EncryptedBlob,
//     pub(crate) files: Vec<EncryptedBlob>,
// }

// #[derive(Default, Serialize, Deserialize)]
// pub(crate) struct UnlockedChest {
//     pub(crate) header: Header,
//     pub(crate) files: Vec<EncryptedBlob>,
// }

// #[derive(Default, Serialize, Deserialize)]
// pub(crate) struct Header {
//     pub(crate) compression_algorithm: Option<CompressionAlgorithm>,
//     pub(crate) key_derivation_algorithm: KeyDerivationAlgorithm,
//     pub(crate) encryption_algorithm: EncryptionAlgorithm,
// }

// impl UnlockedChest {
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

//     pub(crate) fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
//         let serialized = bincode::serialize(self)?;
//         let mut file = File::create(path)?;
//         file.write_all(&serialized)?;
//         Ok(())
//     }

//     pub(crate) fn open_from_payload(payload: &[u8]) -> Result<Self> {
//         Ok(bincode::deserialize(payload)?)
//     }

//     pub(crate) fn open_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
//         let mut file = File::open(path)?;
//         let mut payload = Vec::new();
//         file.read_to_end(&mut payload)?;
//         Self::open_from_payload(&payload)
//     }
// }
