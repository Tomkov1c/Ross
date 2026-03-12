pub mod path;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum GlobalCommands {
    #[command(about = "This is Bob's description", hide = true)]
    Path { },

}
