use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

pub fn add_to_gitignore(gitignore_path: PathBuf, dir: &str) -> io::Result<()> {
    let entry = format!("{}/", dir.trim_end_matches('/'));

    if gitignore_path.exists() {
        let file = fs::File::open(&gitignore_path)?;
        let already_present = io::BufReader::new(file)
                                .lines()
                                .filter_map(|l| l.ok())
                                .any(|line| line.trim() == entry || line.trim() == dir);

        if already_present {
            return Ok(());
        }
    }

    let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&gitignore_path)?;

    let content = fs::read(&gitignore_path)?;
    if !content.is_empty() && content.last() != Some(&b'\n') {
        writeln!(file)?;
    }

    writeln!(file, "{}", entry)?;
    Ok(())
}

pub fn search_dir_for_gitignore(dir: &PathBuf) -> bool {
    dir.join(".gitignore").exists()
}

pub fn add_cache_to_gitignore(dir: PathBuf) -> bool {
    if search_dir_for_gitignore(&dir) {
        add_to_gitignore(dir.join(".gitignore"), ".ross/cache").is_ok()
    } else {
        false
    }
}

pub fn add_config_dir_to_gitignore(dir: PathBuf) -> bool {
    if search_dir_for_gitignore(&dir) {
        add_to_gitignore(dir.join(".gitignore"), ".ross").is_ok()
    } else {
        false
    }
}