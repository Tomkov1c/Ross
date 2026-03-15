use clap::Parser;

use crate::commands::MainCommands;


#[derive(Parser)]
#[command(name = "Ross", about = "Code formatter", version, disable_version_flag = true)]
pub struct Cli {

    #[arg(short = 'v', long, action = clap::ArgAction::Version, help = "Print version information")]
    pub version: Option<bool>,

    #[command(subcommand)]
    pub command: Option<MainCommands>,
}