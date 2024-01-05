use anyhow::Result;
use ring::rand::{SecureRandom, SystemRandom};

pub(crate) fn generate_random_bytes(length: usize) -> Result<Vec<u8>> {
    let rng = SystemRandom::new();
    let mut bytes = vec![0u8; length];
    rng.fill(&mut bytes)?;
    Ok(bytes)
}
