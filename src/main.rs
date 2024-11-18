use std::{error::Error, io::{stdin, Read}, path::Path};

use clap::Parser;
use cli::Args;

pub mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    let Args { files } = Args::parse();

    // If no files are provided, read from STDIN
    let files = if files.is_empty() {
        let mut content = String::new();

        stdin().read_to_string(&mut content)?;

        vec![("STDIN".to_string(), content)]
    } else {
        read_all(files)?
    };

    for (path, content) in files {
        let mailbox = papr::parser::mailbox::Mailbox::try_from(content.as_str())?;
        dbg!(&mailbox);
        println!("{}:\n{}", path, mailbox);
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