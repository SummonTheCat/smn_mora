// src/io_write.rs

use std::{
    fs::File,
    io::{self, Write},
};
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::{RngCore, SeedableRng};

use crate::smn_safe::{safe_io::io_crypto::key_to_seed, safe_structs::SmnSafe};


/// Encrypt & write the safe’s content to `safe.location`.
pub fn write_safe(safe: &SmnSafe) -> io::Result<()> {
    // seed PRNG from key
    let seed = key_to_seed(safe.get_key());
    let mut rng = ChaCha8Rng::from_seed(seed);

    // build keystream & XOR
    let mut keystream = vec![0u8; safe.get_content().len()];
    rng.fill_bytes(&mut keystream);
    let ciphertext: Vec<u8> = safe
        .get_content()
        .iter()
        .zip(keystream.iter())
        .map(|(b, k)| b ^ k)
        .collect();

    // Ensure the directory exists
    if let Some(parent) = safe.get_location().parent() {
        std::fs::create_dir_all(parent)?;
    }

    // write ciphertext to file
    let mut f = File::create(&safe.get_location())?;
    f.write_all(&ciphertext)?;
    Ok(())
}
