use clap::Parser;
use clap::Subcommand;


#[derive(Subcommand)]
pub enum LocalCommands {


}

pub fn match_command(subcommand: Option<LocalCommands>) {
    match subcommand {

        None => { crate::handlers::cli_handler::Cli::parse_from(["", "config", "global", "--help"]); }
    }
}