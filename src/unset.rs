use std::error::Error;

use clap::Parser;
use toml_edit::Document;

#[derive(Parser, Debug)]
pub struct Unset {
    key: String,
}

impl Unset {
    pub fn run(&self, doc: &mut Document) -> Result<(), Box<dyn Error>> {
        // doc["a"]["b"]["c"]["d"] = value("hello");
        Ok(())
    }
}
