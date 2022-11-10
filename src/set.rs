use std::error::Error;

use clap::Parser;
use toml_edit::Document;

#[derive(Parser, Debug)]
pub struct Set {
    key: String,
    val: String,
}

impl Set {
    pub fn run(&self, doc: &mut Document) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
