use std::{env, io};
pub mod config;
pub mod smn_safe;
pub mod smn_interface;

fn main() -> io::Result<()> {
    // load or initialize config (writes default if missing)
    let cfg = config::Config::load_or_init()?;

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        smn_interface::interface_interactive::run_interactive(&cfg)
    } else {
        smn_interface::interface_cli::run_cli(&cfg)
    }
}
