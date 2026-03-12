pub mod bob;
pub mod init;
pub mod config;

use clap::Subcommand;

use crate::commands::config::ConfigCommands;


#[derive(Subcommand)]
pub enum MainCommands {

    #[command(about = "This is Bob's description", hide = true)]
    Bob { },

    #[command(about = "Initialize empty project config directory for Ross. Add cached ross files to .gitignore")]
    Init {
        #[arg(short = 'g', long, help = "Do not touch .gitignore at all", conflicts_with = "gitignore")]
        gitless: bool,

        #[arg(short = 'i', long, help = "Add the whole project config directory to .gitignore", conflicts_with = "gitless")]
        gitignore: bool,
    },

    #[command(about = "Manage Ross configuration")]
    Config {
        #[command(subcommand)]
        subcommand: Option<ConfigCommands>,
    },
}
