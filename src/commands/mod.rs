use clap::Subcommand;

use crate::commands::config::ConfigCommands;

pub mod bob;
pub mod init;
pub mod config;
pub mod tidy;

#[derive(Subcommand)]
pub enum MainCommands {

    #[command(about = "Output a Bob Ross quote", hide = true)]
    Bob {
        #[arg(short, long, help = "Listen to the quote immediately")]
        listen: bool,
    },

    #[command(about = "Initialize empty project config directory for Ross. Add cached ross files to .gitignore")]
    Init {
        #[arg(short = 'g', long, help = "Do not touch .gitignore at all", conflicts_with = "gitignore")]
        gitless: bool,

        #[arg(short = 'i', long, help = "Add the whole project config directory to .gitignore", conflicts_with = "gitless")]
        gitignore: bool,
    },

    #[command(about = "Run code formatting on the attached file(s)")]
    Tidy {
        #[arg(index = 1)]
        extension: Option<String>,

        #[arg(index = 2, requires = "extension")]
        scheme: Option<String>,

        #[arg(long, short, num_args = 1.., conflicts_with = "recursive")]
        files: Vec<String>,

        #[arg(long, short, conflicts_with = "files")]
        recursive: bool,
    },

    #[command(about = "Manage Ross configuration")]
    Config {
        #[command(subcommand)]
        subcommand: Option<ConfigCommands>,
    },
}


pub fn match_command(command: Option<MainCommands>) {
    match command {
        Some(MainCommands::Bob {listen}) => bob::main(listen),
        Some(MainCommands::Init { gitless, gitignore }) => init::main(gitless, gitignore),

        Some(MainCommands::Tidy { extension, scheme, files, recursive }) => tidy::main(),


        // Branched
        Some(MainCommands::Config { subcommand }) => config::match_command(subcommand),
        //


        None => { }
    }
}