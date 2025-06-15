use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    str,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
use clap::{Parser, Subcommand};
use rpassword::read_password;

use crate::smn_safe::safe_structs::SmnSafe;
pub mod smn_safe;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        location: PathBuf,
        key: String,
        content: String,
    },
    Read {
        location: PathBuf,
        key: String,
    },
    Update {
        location: PathBuf,
        key: String,
        content: String,
    },
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        interactive_mode()?;
    } else {
        let cli = Cli::parse();
        match cli.command.expect("subcommand required") {
            Commands::Create { location, key, content } => {
                let mut safe = SmnSafe::new(key, &location);
                safe.set_content(content.into_bytes());
                safe.save()?;
                println!("✅ Created safe and wrote to {:?}", location);
            }
            Commands::Read { location, key } => {
                let mut safe = SmnSafe::new(key, &location);
                safe.load()?;
                let out = str::from_utf8(safe.get_content()).unwrap_or("<binary>");
                println!("\n🔓 Content:\n{}\n🔐 Loaded from {:?}", out, location);
            }
            Commands::Update { location, key, content } => {
                let mut safe = SmnSafe::new(key, &location);
                safe.load()?;
                safe.set_content(content.into_bytes());
                safe.save()?;
                println!("✏️  Updated safe and wrote to {:?}", location);
            }
        }
    }

    Ok(())
}

fn interactive_mode() -> io::Result<()> {
    // top separator
    println!("---------------");

    // 1) Location
    print!("Enter safe file path: ");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let location = PathBuf::from(buf.trim());

    // 2) Key (hidden, with spinner)
    let key = {
        let prompt = "Enter key (hidden): ";
        print!("{}", prompt);
        io::stdout().flush()?;

        let running = Arc::new(AtomicBool::new(true));
        let spinner_flag = running.clone();
        let prompt_string = prompt.to_string();
        let handle = thread::spawn(move || {
            let frames = ['|', '/', '-', '\\'];
            let mut idx = 0;
            while spinner_flag.load(Ordering::SeqCst) {
                print!("\r{} {}", prompt_string, frames[idx % frames.len()]);
                io::stdout().flush().unwrap();
                idx += 1;
                thread::sleep(Duration::from_millis(100));
            }
        });

        let pw = read_password().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        running.store(false, Ordering::SeqCst);
        handle.join().unwrap();
        print!("\r{}   \r", prompt);
        io::stdout().flush()?;

        pw
    };

    // 3) Choose action
    let choice = loop {
        print!("Choose [r]ead or [w]rite: ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        match buf.trim().to_lowercase().as_str() {
            "r" | "read" => break "read",
            "w" | "write" => break "write",
            _ => println!("Please enter 'r' or 'w'."),
        }
    };

    // separator before result
    println!("---------------");

    // 4) Execute
    if choice == "read" {
        let mut safe = SmnSafe::new(key.clone(), &location);
        match safe.load() {
            Ok(()) => {
                if let Ok(text) = str::from_utf8(safe.get_content()) {
                    println!("{}", text);
                } else {
                    // Incorrect key or currupted data notification
                    println!("❌ Failed to decrypt content: Incorrect key or corrupted data");
                }
            }
            Err(e) => eprintln!("❌ Failed to read safe: {}", e),
        }
    } else {
        print!("Enter content to write: ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let mut safe = SmnSafe::new(key.clone(), &location);
        safe.set_content(buf.trim().as_bytes().to_vec());
        match safe.save() {
            Ok(()) => println!("✅ Written and encrypted to {:?}", location),
            Err(e) => eprintln!("❌ Failed to write safe: {}", e),
        }
    }

    Ok(())
}
