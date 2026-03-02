use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Ross", about = "Code formatter", version, disable_version_flag = true)]
pub struct Cli {

    #[arg(short = 'v', long, action = clap::ArgAction::Version, help = "Print version information")]
    pub version: Option<bool>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Bob { },
}
