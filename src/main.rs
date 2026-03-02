mod cli;
use cli::{Cli, Commands};

use clap::{Parser};


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => { println!("Hello, {}!", name.unwrap_or("world".to_string())); }
        Commands::Add { a, b } => { println!("{} + {} = {}", a, b, a + b); }
    }
}
