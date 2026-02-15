use std::{env, fs, process};

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    let contents =
        fs::read_to_string(&config.filename).expect("Something went wrong reading the file");

    for line in contents.lines() {
        if line.contains(&config.query) {
            println!("{}", line)
        }
    }
}
