use std::{env, process};
use std::time::{Instant, Duration};
use raylib::prelude::*;

// Include cpu.rs
mod cpu;

// Resources:
//   - https://austinmorlan.com/posts/chip8_emulator/
//   - https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
//   - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Annn


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

    let window_title = format!("Chip - {}", &args[1]);
    // let (mut rl, thread) = raylib::init()
    //     .size(640, 480)
    //     .title(window_title.as_str())
    //     .build();

    // At 500hz, CPU should cycle every 2 milliseconds.
    // 1 sec / 500 times = 0.002 secs, or 2 ms
    let frequency_hz  = 500;
    let tick_interval = Duration::from_secs(1) / frequency_hz;


    // TODO: Handle window rendering later
    let mut next_tick = Instant::now();
    loop {
        // Run a cycle, then wait until delay has passed to run again
        chip8.cycle();

        // Determine the next tick deadline, then wait until then
        next_tick += tick_interval;

        let now = Instant::now();
        if next_tick > now {
            // Tick deadline - the current time = duration to sleep for
            std::thread::sleep(next_tick - now);
        } else {
            // If a cycle takes longer than 2ms, snap the next tick
            // so we don't try to catch up
            next_tick = now;
        }

        // TODO: CPU execution loop
        // CHIP-8 games expect CPU speed of ~500-700Hz (instructions per second).
        // Need to build a loop that triggers a chip8 cycle at the correct speed.
        // Calculate time needed between each instruction and sleep if too quick.
        // See time::Instant and time::Duration
    }

    // --- Main window loop -----------------------------------------------------
    // while !rl.window_should_close() {
    //     rl.begin_drawing(&thread);

    // }
    // --------------------------------------------------------------------------



    // TODO:
    // - Master clock
    //   - Loop to call cpu.run_cycle() at 500Hz
    //   - Calling cpu.tick_timers() at 60Hz
    // - Keyboard input
    // - Audio output
}
