use clap::{Parser, Subcommand};

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

    #[command(about = "This is Bob's description")]
    Bob { },
}
