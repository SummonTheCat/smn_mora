// src/smn_safe/io_file.rs

use std::{fs, io, path::{Path}};
use crate::smn_safe::safe_structs::SmnSafe;

/// Read an *unencrypted* file into a new `SmnSafe`.
///
/// - `location`: any path to read from  
/// - `key`: the key you want stored in the Safe (no decryption is attempted)  
///
/// # Errors
/// Fails if the file can’t be opened or read.
pub fn new_safe_from_file<P, K>(location: P, key: K) -> io::Result<SmnSafe>
where
    P: AsRef<Path>,
    K: Into<String>,
{
    let bytes = fs::read(&location)?;
    let mut safe = SmnSafe::new(key.into(), location.as_ref().to_path_buf());
    safe.set_content(bytes);

    Ok(safe)
}

/// Save the content of a `SmnSafe` to a file.
///     
/// - `safe`: the `SmnSafe` instance to save
/// - `location`: the path where the safe should be saved
/// 
/// # Errors
/// Fails if the file can’t be written to.
pub fn save_safe_to_file<P>(safe: &SmnSafe, location: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let bytes = safe.get_content();
    fs::write(location, bytes)?;
    Ok(())
}
