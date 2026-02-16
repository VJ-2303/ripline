use std::io::{self, BufRead};
use std::{error::Error, fs::File};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.filename)?;
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
