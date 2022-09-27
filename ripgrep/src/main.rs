use ripgrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    println!("In file {}", config.filename);

    if let Err(e) = ripgrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

