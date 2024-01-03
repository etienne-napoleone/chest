use std::fs;
use std::fs::Metadata;
use std::path::PathBuf;

use anyhow::Result;

use crate::compression::Compress;

pub(crate) struct File {
    path: PathBuf,
}

impl File {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub(crate) fn _metadata(&self) -> Result<Metadata> {
        Ok(fs::metadata(&self.path)?)
    }

    pub(crate) fn compress(&self, compressor: &impl Compress) -> Result<Vec<u8>> {
        compressor.compress(&self.path)
    }
}
