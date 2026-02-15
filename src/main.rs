use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Not enough arguments provided");
    }

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
