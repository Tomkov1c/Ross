use crate::CURRENT_DIR;
use crate::handlers::file_handler;
use crate::handlers::local_config_handler;
use crate::handlers::output_handler;

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
        output_handler::error("Could not get current directory")
    }else {
        output_handler::error(&format!("Initialized empty Ross project config directory at: {}", success.unwrap().parent().unwrap().display().to_string()));
    }
}
