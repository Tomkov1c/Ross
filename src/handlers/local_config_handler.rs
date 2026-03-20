use std::fs;
use std::path::PathBuf;

use crate::CURRENT_DIR;

#[cfg(target_os = "windows")]
use crate::handlers::dir_handler::set_dir_as_hidden_on_windows;

pub fn search_dir_for_local_config(path: PathBuf) -> bool {
    let ross_dir = path.join(".ross");

    ross_dir.exists()
}

pub fn search_env_dir_for_local_config() -> bool {
    search_dir_for_local_config(CURRENT_DIR.clone())
}

pub fn create_local_config(dir: PathBuf) -> Option<PathBuf> {
    if !search_dir_for_local_config(dir.clone()) {
        let ross_dir = dir.join(".ross");
        match fs::create_dir(&ross_dir){
            Ok(_) => crate::debug!("Created .ross dir at {:?}", ross_dir),
            Err(e) => crate::debug!("Failed to create .ross dir: {e}"),
        }

        #[cfg(target_os = "windows")]
        set_dir_as_hidden_on_windows(&ross_dir);

        Some(ross_dir)
    }else {
        None
    }
}

pub fn create_local_config_at_env() -> Option<PathBuf> {
    create_local_config(CURRENT_DIR.clone())
}

pub fn search_through_parent_for_local_config(dir: PathBuf) -> Option<PathBuf> {
    if search_dir_for_local_config(dir.clone()) {
        Some(dir)
    } else {
        search_through_parent_for_local_config(dir.parent()?.to_path_buf())
    }
}

pub fn add_dir_to_config_dir(dir: PathBuf, name: String) -> bool {
    if search_dir_for_local_config(dir.clone()) {
        let named_dir = dir.join(".ross").join(&name);
        if named_dir.exists() {
            false
        } else {
            match fs::create_dir(&named_dir) {
                Ok(_) => crate::debug!("Added a subdir at {:?}", named_dir),
                Err(e) => crate::debug!("Failed to create .a sub dir: {e}"),
            }
            true
        }
    } else {
        false
    }
}