use cli::{Cli, Commands};
use clap::{Parser};

mod commands;
mod cli;


fn main() {
    cli_command_match();
}




// Private
fn cli_command_match() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Bob { } => commands::bob::run(),
    }
}
