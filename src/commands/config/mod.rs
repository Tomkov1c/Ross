use clap::Parser;
use clap::Subcommand;

pub mod global;
pub mod local;

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

pub fn match_command(subcommand: Option<ConfigCommands>) {
    match subcommand {
        Some(ConfigCommands::Global { subcommand }) => global::match_command(subcommand),
        Some(ConfigCommands::Local { subcommand })  => local::match_command(subcommand),

        None => { crate::handlers::cli_handler::Cli::parse_from(["", "config", "--help"]); }
    }
}