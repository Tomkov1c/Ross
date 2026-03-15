pub mod path;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum GlobalCommands {
    #[command(about = "This is Bob's description", hide = true)]
    Path { },

}

pub fn match_command(subcommand: Option<GlobalCommands>) {
    match subcommand {
        Some(GlobalCommands::Path {}) => path::run(),

        None => { crate::handlers::cli_handler::Cli::parse_from(["", "config", "global", "--help"]); }
    }
}