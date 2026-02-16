use clap::Parser;
use std::io::{self, BufRead};
use std::{error::Error, fs::File};

/// Simple program to search for a string in a file
#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Config {
    /// The string to search for
    pub query: String,

    /// The file to search in
    pub filename: String,

    /// Ignore case distinctions (flag: -i or --ignore-case)
    #[arg(short, long)]
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.filename)?;
    let reader = io::BufReader::new(file);

    let query_lower = config.query.to_lowercase();

    for line_result in reader.lines() {
        let line = line_result?;

        let matches = if config.ignore_case {
            line.to_lowercase().contains(&query_lower)
        } else {
            line.contains(&config.query)
        };
        if matches {
            println!("{}", line);
        }
    }
    Ok(())
}
