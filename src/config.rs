use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use serde_json::{Value};

#[derive(Debug)]
pub struct Config(Value);

impl Config {
    pub fn new(filename: &str) -> Result<Config, Box<dyn Error>> {
        let mut file = File::open(filename)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;
        let json = serde_json::from_str(&buffer)?;

        Ok(Config(json))
    }

    pub fn value(&self) -> &Value {
        &self.0
    }
}