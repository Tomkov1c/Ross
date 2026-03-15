use std::path::PathBuf;

pub fn create_or_get_config_dir() -> PathBuf {
    let dir = dirs::config_dir().unwrap().join("ross");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn get_config_file_path(file_name: String) -> PathBuf {
    create_or_get_config_dir().join(file_name + ".json")
}

pub fn create_or_get_schemes_dir() -> PathBuf {
    let dir = create_or_get_config_dir().join("schemes");
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn create_or_get_language_dir(language: String) -> PathBuf {
    let dir = create_or_get_schemes_dir().join(language.to_lowercase());
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

pub fn get_scheme_path_by_name(language: String, name: String) -> PathBuf {
    create_or_get_language_dir(language.to_lowercase()).join(name + ".json")
}