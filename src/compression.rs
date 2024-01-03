use std::io::{Read, Write};

use anyhow::Result;
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;

pub(crate) trait Compress {
    fn compress(&self, payload: Vec<u8>) -> Result<Vec<u8>>;
    fn decompress(&self, payload: Vec<u8>) -> Result<Vec<u8>>;
}

#[derive(Default)]
pub(crate) struct DeflateCompressor;

impl Compress for DeflateCompressor {
    fn compress(&self, payload: Vec<u8>) -> Result<Vec<u8>> {
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&payload)?;
        Ok(encoder.finish()?)
    }

    fn decompress(&self, payload: Vec<u8>) -> Result<Vec<u8>> {
        let mut decoder = DeflateDecoder::new(&payload[..]);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAYLOAD: &[u8; 9] = b"aaaaaaaaa";
    const COMPRESSOR: DeflateCompressor = DeflateCompressor;

    #[test]
    fn compressed_then_decompressed_data_is_similar() {
        let compressed = COMPRESSOR.compress(PAYLOAD.to_vec()).unwrap();
        let decompressed = COMPRESSOR.decompress(compressed).unwrap();
        assert_eq!(PAYLOAD.to_vec(), decompressed);
    }

    #[test]
    fn compressed_data_is_different() {
        let compressed = COMPRESSOR.compress(PAYLOAD.to_vec()).unwrap();
        assert_ne!(PAYLOAD.to_vec(), compressed);
    }

    #[test]
    fn compressed_data_is_smaller() {
        let compressed = COMPRESSOR.compress(PAYLOAD.to_vec()).unwrap();
        assert!(PAYLOAD.to_vec().len() > compressed.len());
    }
}
