pub mod path;

use clap::Subcommand;

pub fn default() {

}

#[derive(Subcommand)]
pub enum ConfigCommands {
    #[command(about = "Print the path to the config file")]
    Path {},
}
