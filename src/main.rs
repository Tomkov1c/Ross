mod commands;
mod handlers;
mod languages;

use crate::handlers::cli_handler::{Cli, Commands};
use crate::commands::config::ConfigCommands;

use clap::Parser;

fn main() {
    cli_match();
}

fn cli_match() {
    let cli = Cli::parse();

    if !cli.files.is_empty() {
        for file in &cli.files {
            println!("Processing file: {}", file);
        }
        return;
    }

    match cli.command {
        Some(Commands::Bob {}) => commands::bob::run(),
        Some(Commands::Init { gitless, gitignore }) => commands::init::main(gitless, gitignore),
        Some(Commands::Config { subcommand }) => match subcommand {
            Some(ConfigCommands::Path {}) => commands::config::path::run(),
            None => commands::config::default(),
        },

        None => { Cli::parse_from(["", "--help"]); }
    }
}
