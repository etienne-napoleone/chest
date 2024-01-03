use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use flate2::write::DeflateEncoder;
use flate2::Compression;

pub(crate) trait Compress {
    fn compress(&self, path: &PathBuf) -> Result<Vec<u8>>;
}

#[derive(Default)]
pub(crate) struct DeflateCompressor {}

impl Compress for DeflateCompressor {
    fn compress(&self, path: &PathBuf) -> Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        file.read_to_end(&mut buffer)?;
        encoder.write_all(&buffer)?;
        Ok(encoder.finish()?)
    }
}
