use std::fs;
use std::fs::Metadata;
use std::path::PathBuf;

use anyhow::Result;

use crate::compression::Compressor;

pub(crate) struct File {
    path: PathBuf,
}

impl File {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub(crate) fn metadata(&self) -> Result<Metadata> {
        Ok(fs::metadata(&self.path)?)
    }

    pub(crate) fn compress(&self, compressor: &impl Compressor) -> Result<Vec<u8>> {
        Ok(compressor.compress(&self.path)?)
    }
}
