use crate::handlers::output_handler;
use crate::handlers::global_config_handler;

pub fn run() {
    output_handler::error(&global_config_handler::create_or_get_config_dir().display().to_string());
}