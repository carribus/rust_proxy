mod config;

use std::error::Error;
use std::env;
use colored::*;

use crate::config::Config;

const CONFIG_FILENAME: &str = "config/config.json";

fn main() {
    if let Err(err) = run_main() {
        println!("{} {}", "ERROR: ".bright_red(), err);
    }
}

fn run_main() -> Result<(), Box<dyn Error>> {
    let path = env::current_dir()?;
    let config = Config::new(&format!("{}/{}", path.to_str().unwrap(), CONFIG_FILENAME))?;

    println!("config = {:#?}", config);
    println!("config.o.key = {:?}", config.value()["o"]["key"]);

    Ok(())
}
