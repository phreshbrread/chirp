#![allow(unused)]

use std::{env, process};
use raylib::prelude::*;

// Include cpu.rs
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

    //  let window_title = format!("Chip - {}", &args[1]);
    //  let (mut rl, thread) = raylib::init()
    //      .size(640, 480)
    //      .title(window_title.as_str())
    //      .build();

    //  // --- Main window loop -----------------------------------------------------
    //  while !rl.window_should_close() {
    //      rl.begin_drawing(&thread);
    //  }
    //  // --------------------------------------------------------------------------

    // TODO:
    // - Raylib window
    // - Keyboard input
    // - Audio output
    // - Master clock
    //   - Loop to call cpu.run_cycle() at 500Hz
    //   - Calling cpu.tick_timers() at 60Hz
}
