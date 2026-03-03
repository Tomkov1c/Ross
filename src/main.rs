use cli::{Cli, Commands};
use clap::Parser;

mod commands;
mod cli;
mod config;

fn main() {
    cli_command_match();
}

fn cli_command_match() {
    let cli = Cli::parse();

    if !cli.files.is_empty() {
        for file in &cli.files {
            println!("Processing file: {}", file);
        }
        return;
    }

    match cli.command {
        Some(Commands::Bob {}) => commands::bob::run(),
        Some(Commands::Init {}) => commands::init::run(),

        None => { Cli::parse_from(["", "--help"]); }
    }
}
