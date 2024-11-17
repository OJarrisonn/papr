use std::{error::Error, path::Path};

use clap::Parser;
use cli::Args;

pub mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let Args { files } = Args::parse();

    if files.is_empty() {
        eprintln!("papr: no files specified STDIN not supported yet");
        return Ok(());
    }

    let files = read_all(files)?;

    for (path, content) in files {
        let mailbox = papr::parser::mailbox::Mailbox::try_from(content.as_str())?;
        dbg!(path, mailbox);
    }

    Ok(())
}

fn read_all(files: Vec<String>) -> Result<Vec<(String, String)>, std::io::Error> {
    files.into_iter().map(|file| {
        let path = Path::new(&file);
        let content = std::fs::read_to_string(path)?;

        Ok((file, content))
    }).collect()
}