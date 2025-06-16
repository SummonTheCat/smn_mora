use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    str,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use crate::config::Config;
use crate::smn_safe::safe_structs::SmnSafe;
use rpassword::read_password;

/// Entry point for interactive (no-args) mode
pub fn run_interactive(cfg: &Config) -> io::Result<()> {
    println!("---------------");

    // 1. Loop until a read or write action is chosen
    let action = loop {
        print!("Choose [r]ead, [w]rite, or [l]ist safes: ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        match buf.trim().to_lowercase().as_str() {
            "r" | "read" => break "read",
            "w" | "write" => break "write",
            "l" | "list" => {
                println!("\nSafes under {}:", cfg.safe_location.display());
                fn walk_and_print(base: &PathBuf, dir: &PathBuf) -> io::Result<()> {
                    for entry in fs::read_dir(dir)? {
                        let entry = entry?;
                        let path = entry.path();
                        if path.is_dir() {
                            walk_and_print(base, &path)?;
                        } else if let Ok(rel) = path.strip_prefix(base) {
                            println!("  {}", rel.display());
                        }
                    }
                    Ok(())
                }
                walk_and_print(&cfg.safe_location, &cfg.safe_location)?;
                println!("---------------");
                continue;
            }
            _ => {
                println!("Please enter 'r', 'w', or 'l'.");
                continue;
            }
        }
    };

    println!("---------------");

    // 2. Prompt for file path (relative to base)
    print!("Enter safe file path (relative to base): ");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let rel_path = PathBuf::from(buf.trim());
    let full_path = cfg.safe_location.join(&rel_path);

    // 3. Prompt for key (hidden with spinner)
    let key = {
        let prompt = "Enter key (hidden): ";
        print!("{}", prompt);
        io::stdout().flush()?;

        let running = Arc::new(AtomicBool::new(true));
        let spinner_flag = running.clone();
        let prompt_text = prompt.to_string();

        let handle = thread::spawn(move || {
            let frames = ['|', '/', '-', '\\'];
            let mut idx = 0;
            while spinner_flag.load(Ordering::SeqCst) {
                print!("\r{} {}", prompt_text, frames[idx % frames.len()]);
                io::stdout().flush().unwrap();
                idx += 1;
                thread::sleep(Duration::from_millis(100));
            }
        });

        let pw = read_password().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        running.store(false, Ordering::SeqCst);
        handle.join().unwrap();
        print!("\r\x1b[K");
        io::stdout().flush()?;

        pw
    };

    println!("---------------");

    // 4. Execute read or write
    if action == "read" {
        let mut safe = SmnSafe::new(key.clone(), &full_path);
        match safe.load() {
            Ok(()) => {
                if let Ok(text) = str::from_utf8(safe.get_content()) {
                    println!("{}", text);
                } else {
                    println!("❌ Failed to decrypt content: Incorrect key or corrupted data");
                }
            }
            Err(e) => eprintln!("❌ Failed to read safe: {}", e),
        }
    } else {
        // write
        print!("Enter content to write (or `file:/absolute/path`): ");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        let input = buf.trim();

        // If input starts with `file:`, read that file's contents
        let content_bytes = if let Some(path_str) = input.strip_prefix("file:") {
            let file_path = PathBuf::from(path_str);
            match fs::read(&file_path) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("❌ Failed to read from file '{}': {}", path_str, e);
                    return Ok(());
                }
            }
        } else {
            input.as_bytes().to_vec()
        };

        let mut safe = SmnSafe::new(key.clone(), &full_path);
        safe.set_content(content_bytes);
        match safe.save() {
            Ok(()) => println!("✅ Written and encrypted to {:?}", full_path),
            Err(e) => eprintln!("❌ Failed to write safe: {}", e),
        }
    }

    Ok(())
}
