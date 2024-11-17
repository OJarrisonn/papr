use clap::Parser;
use cli::Args;

pub mod cli;

fn main() {
    Args::parse();

    println!("papr init, come back later");
}
