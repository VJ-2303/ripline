use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Not enough arguments provided");
    }

    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("Searching for: {}", query);
    println!("In file: {}", filename);
    println!("--- Results ---");

    for line in contents.lines() {
        if line.contains(query) {
            println!("{}", line)
        }
    }
}
