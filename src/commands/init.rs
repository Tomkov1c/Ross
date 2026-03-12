use crate::{CURRENT_DIR, handlers::{file_handler, local_config_handler}};
use std::env;

pub fn main(gitless: bool, gitignore: bool) {
    let success = local_config_handler::create_local_config_at_env();

    if !gitless {
        if gitignore {
            file_handler::add_config_dir_to_gitignore(CURRENT_DIR.clone());
        } else {
            file_handler::add_cache_to_gitignore(CURRENT_DIR.clone());
        }
    }

    if success == None {
        println!("Could not get current directory")
    }else {
        println!("Initialized empty Ross project config directory at: {}", success.unwrap().parent().unwrap().display().to_string());
    }
}
