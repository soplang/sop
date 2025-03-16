use clap::{Parser, Subcommand};

mod cli;
mod commands;
mod toml_parser;
mod utils;

use crate::cli::Cli;

fn main() {
    let cli = Cli::parse();

    match cli.execute() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
