use clap::Parser;
use clap::Subcommand;

pub mod path;

#[derive(Subcommand)]
pub enum ConfigCommands {
    #[command(about = "Return the path of the config dir")]
    Path {
        #[arg(short, long, help = "Apply command to the machine config", conflicts_with = "local")]
        global: bool,
        #[arg(short, long, help = "Apply command to the local config", conflicts_with = "global")]
        local: bool,
    },
}

pub fn match_command(subcommand: Option<ConfigCommands>) {
    match subcommand {
        Some(ConfigCommands::Path { global, local }) => path::main(global, local),

        None => { crate::handlers::cli_handler::Cli::parse_from(["", "config", "--help"]); }
    }
}