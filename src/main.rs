mod commands;
mod handlers;

use crate::handlers::cli_handler::{Cli, Commands};
use crate::commands::config::ConfigCommands;

use clap::Parser;

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
        Some(Commands::Config { subcommand }) => match subcommand {
            ConfigCommands::Path {} => commands::config::path::run(),
        },

        None => { Cli::parse_from(["", "--help"]); }
    }
}
