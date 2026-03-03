pub mod path;


use clap::Subcommand;

#[derive(Subcommand)]
pub enum ConfigCommands {
    #[command(about = "Print the path to the config file")]
    Path {},
}