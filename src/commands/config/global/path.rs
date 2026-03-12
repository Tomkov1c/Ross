use crate::handlers::global_config_handler;

pub fn run() {
    println!("{}", global_config_handler::create_or_get_config_dir().display());
}