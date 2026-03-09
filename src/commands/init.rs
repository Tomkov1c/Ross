use crate::handlers::{file_handler, local_config_handler};
use std::env;

pub fn main(gitless: bool, gitignore: bool) {
    let success = local_config_handler::create_local_config_at_env();

    if !gitless {
        let dir = env::current_dir().expect("Could not get current directory");

        if gitignore {
            file_handler::add_config_dir_to_gitignore(dir);
        } else {
            file_handler::add_cache_to_gitignore(dir);
        }
    }

    if success == None {
        println!("Could not get current directory")
    }else {
        println!("Initialized empty Ross project config directory at: {}", success.unwrap().parent().unwrap().display().to_string());
    }
}
