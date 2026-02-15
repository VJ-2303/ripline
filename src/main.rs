use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("We received these arguments: {:?}", args);
}
