mod commands;
mod cli;
use cli::{Cli, Commands};

use clap::{Parser};


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bob { } => commands::bob::run(),
    }
}
