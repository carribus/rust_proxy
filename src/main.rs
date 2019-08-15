mod config;

use std::error::Error;

use crate::config::Config;

fn main() {
    if let Err(err) = run_main() {
        println!("ERROR: {}", err);
    }
}

fn run_main() -> Result<(), Box<dyn Error>> {
    let config = Config {};

    Ok(())
}
