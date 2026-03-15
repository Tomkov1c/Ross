mod commands;
mod handlers;
mod languages;

use crate::commands::config::global::GlobalCommands;
use crate::commands::file::FileCommands;
use crate::handlers::cli_handler::Cli;

use crate::commands::MainCommands;
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

    match cli.command {
        Some(MainCommands::Bob {}) => commands::bob::run(),

        Some(MainCommands::Init { gitless, gitignore }) => commands::init::main(gitless, gitignore),

        Some(MainCommands::Config { subcommand }) => match subcommand {
            Some(ConfigCommands::Global { subcommand }) => match subcommand {
                Some(GlobalCommands::Path {}) => commands::config::global::path::run(),
                None => { Cli::parse_from(["", "config", "global", "--help"]); },
            },

            Some(ConfigCommands::Local { subcommand }) => match subcommand {
                None => { Cli::parse_from(["", "config", "local", "--help"]); },
            },

            None => { Cli::parse_from(["", "config", "--help"]); },
        },

        Some(MainCommands::File { files, subcommand}) => match subcommand {
            Some(FileCommands::Scheme { }) => {  },

            None => {}

        },

        None => { Cli::parse_from(["", "--help"]); }
    }
}
