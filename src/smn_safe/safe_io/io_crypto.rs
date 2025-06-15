// src/crypto.rs

use sha2::{Digest, Sha256};

/// Turn any UTF-8 key into a deterministic 32-byte seed.
pub fn key_to_seed(key: &str) -> [u8; 32] {
    let hash = Sha256::digest(key.as_bytes());
    let mut seed = [0u8; 32];
    seed.copy_from_slice(&hash);
    seed
}
