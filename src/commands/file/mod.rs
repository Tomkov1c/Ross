use clap::Subcommand;

#[derive(Subcommand)]
pub enum FileCommands {

    Scheme { }
}

pub fn match_command(files: &[String], subcommand: Option<FileCommands>) {
    match subcommand {
        Some(FileCommands::Scheme { }) => { },

        None => default(),
    }
}

pub fn default() {

}
