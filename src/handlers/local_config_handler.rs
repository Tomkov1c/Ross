use crate::CURRENT_DIR;

#[cfg(target_os = "windows")]
use crate::handlers::dir_handler::set_dir_as_hidden_on_windows;

use std::fs;
use std::path::PathBuf;

pub fn search_dir_for_local_config(path: PathBuf) -> bool {
    let ross_dir = path.join(".ross");

    ross_dir.exists()
}

pub fn search_env_dir_for_local_config() -> bool {
    search_dir_for_local_config(CURRENT_DIR.clone())
}

pub fn create_local_config_at_env() -> Option<PathBuf> {
    if !search_env_dir_for_local_config() {
        let ross_dir = CURRENT_DIR.join(".ross");
        fs::create_dir(&ross_dir).expect("");

        #[cfg(target_os = "windows")]
        set_dir_as_hidden_on_windows(&ross_dir);

        Some(ross_dir)
    }else {
        None
    }
}

pub fn search_through_parent_for_local_config(dir: PathBuf) -> Option<PathBuf> {
    if search_dir_for_local_config(dir.clone()) {
        Some(dir)
    } else {
        search_through_parent_for_local_config(dir.parent()?.to_path_buf())
    }
}