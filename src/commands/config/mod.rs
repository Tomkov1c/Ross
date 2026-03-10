pub mod global;
pub mod local;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommands {


    #[command(about = "Change settings for your project config")]
    Local {},

    #[command(about = "Change settings for your machine's config")]
    Global {},
}
