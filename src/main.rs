#![allow(unused)]

use std::{env, process};

mod cpu;

// Resources:
//   - https://austinmorlan.com/posts/chip8_emulator/
//   - https://tobiasvl.github.io/blog/write-a-chip-8-emulator/

fn main() {
    // Ensure arg is given
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: chip [ROM]");
        process::exit(1);
    }

    let mut chip8: cpu::Chip8 = cpu::Chip8::new();
    println!("Initialised CPU");

    // Attempt to load ROM from first argument
    chip8.load_rom(&args[1]);

    println!("Hello, world!");

    // TODO:
    // - SDL window
    // - Keyboard input
    // - Audio output
    // - Master clock
    //   - Loop to call cpu.run_cycle() at 500Hz
    //   - Calling cpu.tick_timers() at 60Hz
}
