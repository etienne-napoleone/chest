use thiserror::Error;

pub(crate) type ChestResult<T> = Result<T, ChestError>;

#[derive(Debug, Error)]
pub(crate) enum ChestError {
    #[error("Failed interacting with the filesystem: {0}")]
    Io(#[from] std::io::Error),
    #[error("Couldn't serialize or deserialize: {0}")]
    Serialization(#[from] bincode::Error),
    #[error("Couldn't compress or decompress: {0}")]
    Compress(#[from] CompressError),
    #[error("Couldn't compress or decompress: {0}")]
    Encrypt(#[from] EncryptError),
}

pub(crate) type CompressResult<T> = Result<T, CompressError>;

#[derive(Debug, Error)]
pub(crate) enum CompressError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
}

pub(crate) type EncryptResult<T> = Result<T, EncryptError>;

#[derive(Debug, Error)]
pub(crate) enum EncryptError {
    #[error("Crypto error")]
    Crypto(#[from] ring::error::Unspecified),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
