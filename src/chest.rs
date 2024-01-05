use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::compression::{get_compressor, Compress};
use crate::crypto::{get_encryptor, Encrypt};
use crate::key::{get_deriver, Derive};

#[derive(Serialize, Deserialize)]
pub(crate) struct UnlockedChest {
    key: Vec<u8>,
    public: Public,
    pub(crate) files: Vec<UnlockedFile>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct Public {
    compression_algorithm: Option<CompressionAlgorithm>,
    key_derivation_algorithm: KeyDerivationAlgorithm,
    key_derivation_salt: Vec<u8>,
    encryption_algorithm: EncryptionAlgorithm,
}

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

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct UnlockedFile {
    pub(crate) cipher: EncryptedBlob,
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

#[derive(Serialize, Deserialize)]
pub(crate) struct LockedChest {
    public: Public,
    files: Vec<LockedFile>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct LockedFile {
    pub(crate) cipher: EncryptedBlob,
    pub(crate) metadata: EncryptedBlob,
}

impl UnlockedChest {
    pub(crate) fn new(password: &str, compress: bool) -> Result<Self> {
        let public = Public {
            compression_algorithm: compress.then_some(CompressionAlgorithm::default()),
            ..Public::default()
        };
        let deriver = get_deriver(&public.key_derivation_algorithm);
        let key = deriver.derive(password, &public.key_derivation_salt)?;
        let files = Vec::default();
        Ok(Self { key, public, files })
    }

    pub(crate) fn add_file_from_cipher(
        &mut self,
        cipher: Vec<u8>,
        metadata: Metadata,
    ) -> Result<()> {
        let encryptor = get_encryptor(&self.public.encryption_algorithm);
        let cipher = match &self.public.compression_algorithm {
            Some(compression_algorithm) => {
                let compressor = get_compressor(compression_algorithm);
                compressor.compress(&cipher)?
            }
            None => cipher,
        };
        let file = UnlockedFile {
            cipher: encryptor.encrypt(cipher, &self.key.clone().try_into().unwrap())?,
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
        let mut cipher = Vec::new();
        file.read_to_end(&mut cipher)?;
        self.add_file_from_cipher(cipher, metadata)?;
        Ok(())
    }

    pub(crate) fn lock(self, password: &str) -> Result<LockedChest> {
        let public = self.public;
        let deriver = get_deriver(&public.key_derivation_algorithm);
        let encryptor = get_encryptor(&public.encryption_algorithm);
        let key = deriver.derive(password, &public.key_derivation_salt)?;
        let files = self
            .files
            .into_iter()
            .map(|f| LockedFile {
                cipher: f.cipher,
                metadata: encryptor
                    .encrypt(
                        bincode::serialize(&f.metadata).unwrap(),
                        &key.clone().try_into().unwrap(),
                    )
                    .unwrap(),
            })
            .collect();
        Ok(LockedChest { public, files })
    }

    pub(crate) fn decrypt_files_to_folder<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir_all(&path)?;
        let encryptor = get_encryptor(&self.public.encryption_algorithm);
        let compressor = &self
            .public
            .compression_algorithm
            .as_ref()
            .map(get_compressor);
        self.files.iter().for_each(|f| {
            let binary = encryptor
                .decrypt(&f.cipher, &self.key.clone().try_into().unwrap())
                .unwrap();
            let binary = match compressor {
                Some(compressor) => compressor.decompress(&binary).unwrap(),
                None => binary,
            };
            let file_path = path.as_ref().join(&f.metadata.filename);
            let mut file = fs::File::create(file_path).unwrap();
            file.write_all(&binary).unwrap();
        });
        Ok(())
    }
}

impl LockedChest {
    pub(crate) fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = fs::File::open(path)?;
        let mut payload = Vec::new();
        file.read_to_end(&mut payload)?;
        Ok(bincode::deserialize(&payload)?)
    }

    pub(crate) fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let serialized = bincode::serialize(self)?;
        let mut file = fs::File::create(path)?;
        file.write_all(&serialized)?;
        Ok(())
    }

    pub(crate) fn unlock(self, password: &str) -> Result<UnlockedChest> {
        let public = self.public;
        let deriver = get_deriver(&public.key_derivation_algorithm);
        let encryptor = get_encryptor(&public.encryption_algorithm);
        let key = deriver.derive(password, &public.key_derivation_salt)?;
        let files = self
            .files
            .into_iter()
            .map(|f| UnlockedFile {
                cipher: f.cipher,
                metadata: bincode::deserialize(
                    &encryptor
                        .decrypt(&f.metadata, &key.clone().try_into().unwrap())
                        .unwrap(),
                )
                .unwrap(),
            })
            .collect();
        Ok(UnlockedChest { key, public, files })
    }
}
