// src/io_read.rs

use std::{
    fs::File,
    io::{self, Read},
};
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::smn_safe::{safe_io::io_crypto::key_to_seed, safe_structs::SmnSafe};

/// Read & decrypt from `safe.location` into `safe.content`.
pub fn read_safe(safe: &mut SmnSafe) -> io::Result<()> {
    // load ciphertext
    let mut f = File::open(&safe.get_location())?;
    let mut ciphertext = Vec::new();
    f.read_to_end(&mut ciphertext)?;

    // regenerate keystream
    let seed = key_to_seed(safe.get_key());
    let mut rng = ChaCha8Rng::from_seed(seed);
    let mut keystream = vec![0u8; ciphertext.len()];
    rng.fill_bytes(&mut keystream);

    // decrypt
    let plaintext: Vec<u8> = ciphertext
        .iter()
        .zip(keystream.iter())
        .map(|(c, k)| c ^ k)
        .collect();

    safe.set_content(plaintext);
    Ok(())
}
