use std::env;
use std::fs;

pub fn run() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let new_dir = current_dir.join(".ross");
    fs::create_dir(&new_dir).expect("Failed to create directory");
    println!("Created Ross config directory at: {}", new_dir.display());
}
