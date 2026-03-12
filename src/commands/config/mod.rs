pub mod global;
pub mod local;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommands {

    #[command(about = "Change settings for your project config")]
    Local {
        #[command(subcommand)]
        subcommand: Option<local::LocalCommands>,
    },

    #[command(about = "Change settings for your machine's config")]
    Global {
        #[command(subcommand)]
        subcommand: Option<global::GlobalCommands>,
    },
}