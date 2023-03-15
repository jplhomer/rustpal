use rustpal::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // TODO: Add better error handling for missing API key
    if let Ok(config) = config {
        if let Err(e) = rustpal::run(config) {
            println!("Application error: {}", e);
            process::exit(1);
        }
    } else {
        println!("Problem parsing arguments");
        process::exit(1);
    }
}
