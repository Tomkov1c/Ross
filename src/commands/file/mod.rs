use clap::Subcommand;

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
    if files.is_empty() {
        println!("No files provided.");
        return;
    }

    for file in files {
        println!("Processing file: {}", file);
    }
}