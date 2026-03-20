pub mod bob;
pub mod init;
pub mod config;
pub mod file;

use clap::Subcommand;

use crate::commands::{config::ConfigCommands, file::FileCommands};


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
    File {
            #[arg(required = true)]
            files: Vec<String>,

            #[command(subcommand)]
            subcommand: Option<FileCommands>,
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


        // Branched
        Some(MainCommands::Config { subcommand }) => config::match_command(subcommand),

        Some(MainCommands::File { files, subcommand }) => file::match_command(&files, subcommand),
        //


        None => { }
    }
}