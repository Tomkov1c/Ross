use crate::handlers::local_config_handler;

pub fn main() {
    let success = local_config_handler::create_local_config_at_env();

    if success == None {
        println!("Could not get current directory")
    }else {
        println!("Initialized empty Ross project config directory at: {}", success.unwrap().parent().unwrap().display().to_string());
    }
}
