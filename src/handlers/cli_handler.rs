use clap::{Parser, Subcommand};

use crate::commands::config::ConfigCommands;

#[derive(Parser)]
#[command(name = "Ross", about = "Code formatter", version, disable_version_flag = true)]
pub struct Cli {

    #[arg(short = 'v', long, action = clap::ArgAction::Version, help = "Print version information")]
    pub version: Option<bool>,

    pub files: Vec<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {

    #[command(about = "This is Bob's description", hide = true)]
    Bob { },

    #[command(about = "Initialize empty project config directory for Ross. Adds cached files to .gitignore")]
    Init {
        #[arg(short = 'l', long, help = "Do not touch .gitignore at all", conflicts_with = "gitignore")]
        gitless: bool,

        #[arg(short = 'i', long, help = "Add the whole directory to .gitignore", conflicts_with = "gitless")]
        gitignore: bool,
    },

    #[command(about = "Manage Ross configuration")]
    Config {
        #[command(subcommand)]
        subcommand: Option<ConfigCommands>,
    },
}
