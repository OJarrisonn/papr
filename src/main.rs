use std::{
    io::{stdin, Read},
    path::Path,
};

use clap::Parser;
use cli::Args;
use color_eyre::eyre::{Context, Result};
use papr::parser::mailbox::Mailbox;

pub mod cli;

fn main() -> Result<()> {
    let Args { files, frontmatter } = Args::parse();

    // If no files are provided, read from STDIN
    let files = if files.is_empty() {
        let mut content = String::new();

        stdin()
            .read_to_string(&mut content)
            .with_context(|| "Failed to read input from STDIN")?;

        vec![("STDIN".to_string(), content)]
    } else {
        read_all(files).with_context(|| "While opening files")?
    };

    for (path, content) in files {
        let mut mailbox = Mailbox::try_from(content.as_str())?;

        if frontmatter {
            let messages = mailbox.messages;
            
            mailbox.messages = messages.into_iter().map(|mut message| {
                message.body = message.body.front_matter_only();
                message
            }).collect();
        }
        
        println!("{}:\n{}", path, mailbox);
    }

    Ok(())
}

fn read_all(files: Vec<String>) -> Result<Vec<(String, String)>> {
    files
        .into_iter()
        .map(|file| {
            let path = Path::new(&file);
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("File {} could not be opened", &path.display()))?;

            Ok((file, content))
        })
        .collect()
}
