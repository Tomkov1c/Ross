use clap::Subcommand;

use crate::handlers::output_handler;

#[derive(Subcommand)]
pub enum FileCommands {

    Scheme { }
}

pub fn match_command(files: &[String], subcommand: Option<FileCommands>) {
    match subcommand {
        Some(FileCommands::Scheme { }) => { },

        None => default(files),
    }
}

pub fn default(files: &[String]) {
    output_handler::bar_start(files.len() as u64, "Processing files");

    for file in files {
        output_handler::increase_position(1);
    }

    output_handler::bar_finish("Done");
}