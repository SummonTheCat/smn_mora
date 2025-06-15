use std::path::Path;
/// SmnSafe module containing the core safe structure.
// SmnSafe holds the current set key, and the content in bytes
// It is used to manage the safe's state and operations.
use std::path::PathBuf;

use crate::smn_safe::safe_io::io_read;
use crate::smn_safe::safe_io::io_write;

pub struct SmnSafe {
    key: String,
    location: PathBuf,
    content: Vec<u8>,
}

impl SmnSafe {
    // ----- ----- Lifecycle ----- ----- //
    /// Creates a new SmnSafe instance with an empty key and content.
    pub fn new<K, P>(key: K, location: P) -> Self
    where
        K: Into<String>,
        P: AsRef<Path>,
    {
        SmnSafe {
            key: key.into(),
            location: location.as_ref().to_path_buf(),
            content: Vec::new(),
        }
    }

    // ----- ----- Key Management ----- ----- //
    /// Gets the current key of the safe.
    pub fn get_key(&self) -> &str {
        &self.key
    }

    // ----- ----- Location Management ----- ----- //
    /// Gets the location of the safe.
    pub fn get_location(&self) -> &PathBuf {
        &self.location
    }

    /// Sets the location of the safe.
    pub fn set_location<P: AsRef<Path>>(&mut self, location: P) {
        self.location = location.as_ref().to_path_buf();
    }

    // ----- ----- Content Management ----- ----- //

    /// Gets the current content of the safe.
    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    /// Sets the content of the safe.
    pub fn set_content(&mut self, content: Vec<u8>) {
        self.content = content;
    }

    /// ----- ----- Utility Methods ----- ----- //
    // to_string method to return the key and content as a string
    pub fn to_string(&self) -> String {
        format!("Key: {}, \nContent: \n{:?}", self.key, self.content)
    }

    /// Convenience: call the write module
    pub fn save(&self) -> std::io::Result<()> {
        io_write::write_safe(self)
    }

    /// Convenience: call the read module
    pub fn load(&mut self) -> std::io::Result<()> {
        io_read::read_safe(self)
    }
}
