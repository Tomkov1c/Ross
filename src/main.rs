mod commands;
mod handlers;
mod languages;

use crate::handlers::cli_handler::{Cli, Commands};
use crate::commands::config::ConfigCommands;

use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;

use clap::Parser;

pub static CURRENT_DIR: LazyLock<PathBuf> = LazyLock::new(|| { env::current_dir().expect("Failed to get current directory")});

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
            Some(ConfigCommands::Global {}) => commands::config::global::run(),
            Some(ConfigCommands::Local {}) => commands::config::local::run(),
            None => { Cli::parse_from(["", "--help"]); },
        },

        None => { Cli::parse_from(["", "--help"]); }
    }
}
