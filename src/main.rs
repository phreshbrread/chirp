#![allow(unused)]


use std::env;
use std::process;

mod cpu;

// Resources:
//   - https://austinmorlan.com/posts/chip8_emulator/
//   - https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

fn main() {
    let mut chip8: cpu::Chip8 = cpu::Chip8::new();
    println!("Initialised CPU");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: chip [ROM]");
        process::exit(1);
    }

    println!("Hello, world!");
}

