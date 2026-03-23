use crate::CURRENT_DIR;
use crate::handlers::local_config_handler;
use crate::handlers::output_handler;
use crate::handlers::global_config_handler;

pub fn main(global: bool, local: bool) {

    let result = if global {
        global_config_handler::create_or_get_config_dir().display().to_string()

    } else {
        local_config_handler::search_through_parent_for_local_config(CURRENT_DIR.clone())
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "No local config found".to_string())
    };

    output_handler::info(&result);
}