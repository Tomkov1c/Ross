pub fn run(name: Option<String>) {
    println!("Hello, {}!", name.unwrap_or("world".to_string()));
}
