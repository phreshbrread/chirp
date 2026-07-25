use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: c8a [ROM]");
        process::exit(1);
    }

    println!("Hello, world!");
}
