pub mod bob;
pub mod init;
pub mod config;
pub mod file;

use std::time::Duration;

use clap::Subcommand;

use crate::{commands::{config::ConfigCommands, file::FileCommands}, handlers::output_handler::{self}};


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


        None => {
            output_handler::error("msg");
            output_handler::warn("msg");
            output_handler::info("msg");
            output_handler::normal("msg");

            output_handler::bar_start(50, "Processing");

            for i in 0..50 {
                std::thread::sleep(Duration::from_millis(200));
                output_handler::increase_position(1);
                output_handler::print_below_offset(&format!("last step: {i}"));

                if i == 15 {
                    output_handler::print_above_offset("Checkpoint reached");
                }
            }

            output_handler::finish_offset("Done");
        }
    }
}