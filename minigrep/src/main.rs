use std::{process, env};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)
        .unwrap_or_else(|e| {
            eprintln!("Problem parsing arguments: {}", e);
            process::exit(1);
        });
    println!("Searching for '{}' in file '{}'.", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}