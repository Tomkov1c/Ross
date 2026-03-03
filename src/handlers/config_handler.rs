use std::path::PathBuf;

fn get_or_create_config_dir() -> PathBuf {
    let dir = dirs::config_dir().unwrap().join("ross");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn create_scheme_dir() -> PathBuf {
    let dir = get_or_create_config_dir().join("schemes");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn get_config_path(file_name: &str) -> PathBuf {
    get_or_create_config_dir().join(file_name)
}