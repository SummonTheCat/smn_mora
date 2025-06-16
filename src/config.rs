use serde::{Deserialize, Serialize};
use std::{env, fs, io, path::PathBuf};

/// The shape of our config file.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Absolute path under which all safes live.
    pub safe_location: PathBuf,
}

impl Config {
    /// Load existing config or create a default one (cwd) if none exists.
    pub fn load_or_init() -> io::Result<Self> {
        let path = Config::path()?;
        if path.exists() {
            // parse existing
            let toml = fs::read_to_string(&path)?;
            toml::from_str(&toml)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        } else {
            // create default
            let cwd = env::current_dir()?;
            let cfg = Config { safe_location: cwd };
            cfg.save()?;
            Ok(cfg)
        }
    }

    /// Save this config back to disk.
    pub fn save(&self) -> io::Result<()> {
        let path = Config::path()?;
        let toml = toml::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(path, toml)
    }

    /// Where `config.toml` lives in the OS-standard per-user config dir.
    pub fn path() -> io::Result<PathBuf> {
        let mut dir = dirs::config_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No config dir found"))?;
        dir.push("smn_mora");
        fs::create_dir_all(&dir)?;
        dir.push("config.toml");
        Ok(dir)
    }
}
