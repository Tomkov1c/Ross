use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Ross", about = "My CLI tool", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hello
    Hello {
        /// Name to greet
        name: Option<String>,
    },
    /// Add two numbers
    Add { a: f64, b: f64 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => {
            println!("Hello, {}!", name.unwrap_or("world".to_string()));
        }
        Commands::Add { a, b } => {
            println!("{} + {} = {}", a, b, a + b);
        }
    }
}
