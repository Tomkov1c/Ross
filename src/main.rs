use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;
use clap::Parser;

use crate::handlers::cli_handler::Cli;

mod commands;
mod handlers;
mod languages;

pub static CURRENT_DIR: LazyLock<PathBuf> = LazyLock::new(|| { env::current_dir().unwrap_or_else(|e| { debug!("Failed to get current directory"); std::process::exit(1);})});

fn main() {
    cli_match();
}

fn cli_match() {
    let cli = Cli::parse();
    commands::match_command(cli.command);
}
