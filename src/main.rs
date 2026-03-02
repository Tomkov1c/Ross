mod commands;
mod cli;
use cli::{Cli, Commands};

use clap::{Parser};


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => commands::hello::run(name),
        Commands::Add { a, b }   => commands::add::run(a, b),
    }
}
