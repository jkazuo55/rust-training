use std::env;
use rust_training::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);


    println!("file: {}", config.filename);
    println!("search: {}", config.query);

    rust_training::run(config)
    
}
