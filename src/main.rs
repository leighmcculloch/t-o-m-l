use std::{
    error::Error,
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use toml_edit::Document;

pub const GIT_REVISION: &str = env!("GIT_REVISION");

mod get;
mod set;
mod unset;

#[derive(Parser, Debug)]
#[clap(version, disable_help_subcommand = true, disable_version_flag = true)]
struct Home {
    /// TOML file
    file: PathBuf,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Set a key in the toml file, changing or adding the key to the file.
    Set(set::Set),
    /// Unset a key in the toml file, removing it from the file.
    Unset(unset::Unset),
    /// Get the value of a key in the file.
    Get(get::Get),
}

fn main() -> Result<(), Box<dyn Error>> {
    let home = Home::parse();

    let raw = read_to_string(home.file)?;
    let mut doc = raw.parse::<Document>()?;

    match home.cmd {
        Cmd::Set(set) => set.run(&mut doc)?,
        Cmd::Unset(unset) => unset.run(&mut doc)?,
        Cmd::Get(get) => get.run(&doc)?,
    };

    let new_raw = doc.to_string();
    if raw != new_raw {
        let mut file = File::create("foo.txt")?;
        file.write_all(new_raw.as_bytes())?;
    }
    Ok(())
}
