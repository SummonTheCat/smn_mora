// src/smn_interface/interface_cli.rs

use std::io;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::config::Config;
use crate::smn_safe::safe_structs::SmnSafe;
use std::str;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Operate on a safe (create/read/update)
    #[command(subcommand)]
    Safe(SafeCommand),

    /// View or change configuration
    #[command(subcommand)]
    Config(ConfigCommand),
}

#[derive(Subcommand)]
pub enum SafeCommand {
    Create {
        /// Path relative to the configured base directory
        location: PathBuf,
        key: String,
        content: String,
    },
    Read {
        /// Path relative to the configured base directory
        location: PathBuf,
        key: String,
    },
    Update {
        /// Path relative to the configured base directory
        location: PathBuf,
        key: String,
        content: String,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    /// Show current configuration
    Show,

    /// Set a new base directory for safes
    SetBase {
        /// Absolute or relative path to use as new base
        base: PathBuf,
    },
}

pub fn run_cli(cfg: &Config) -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Safe(cmd) => match cmd {
            SafeCommand::Create { location, key, content } => {
                let full = cfg.safe_location.join(location);
                let mut safe = SmnSafe::new(key, &full);
                safe.set_content(content.into_bytes());
                safe.save()?;
                println!("✅ Created safe at {:?}", full);
            }
            SafeCommand::Read { location, key } => {
                let full = cfg.safe_location.join(location);
                let mut safe = SmnSafe::new(key, &full);
                safe.load()?;
                let out = str::from_utf8(safe.get_content()).unwrap_or("<binary>");
                println!("\n🔓 Content:\n{}\n🔐 Loaded from {:?}", out, full);
            }
            SafeCommand::Update { location, key, content } => {
                let full = cfg.safe_location.join(location);
                let mut safe = SmnSafe::new(key, &full);
                safe.load()?;
                safe.set_content(content.into_bytes());
                safe.save()?;
                println!("✏️  Updated safe at {:?}", full);
            }
        },

        Commands::Config(cmd) => match cmd {
            ConfigCommand::Show => {
                println!("Config file: {:?}", Config::path()?);
                println!("Safe base directory: {:?}", cfg.safe_location);
            }
            ConfigCommand::SetBase { base } => {
                let mut new_cfg = cfg.clone(); // make sure Config derives Clone
                let new_base = if base.is_absolute() {
                    base
                } else {
                    std::env::current_dir()?.join(base)
                };
                new_cfg.safe_location = new_base;
                new_cfg.save()?;
                println!("✅ Updated safe base to {:?}", new_cfg.safe_location);
            }
        },
    }

    Ok(())
}
