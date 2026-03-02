mod commands;
mod cli;
use cli::{Cli, Commands};

use clap::{Parser};


fn main() {
    cli_command_match();
}


fn cli_command_match() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Bob { } => commands::bob::run(),
    }
}
