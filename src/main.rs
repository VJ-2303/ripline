use clap::Parser;
use ripline::Config;
use std::process;

fn main() {
    let config = Config::parse();

    if let Err(e) = ripline::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
