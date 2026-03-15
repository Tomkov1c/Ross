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
    commands::match_command(cli.command);
}
