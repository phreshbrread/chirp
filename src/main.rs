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

    // Set pixel + screen scales
    // TODO: Allow for adjustment
    const SCALE:    i32 = 20;
    const SCREEN_W: i32 = 64 * SCALE;
    const SCREEN_H: i32 = 32 * SCALE;

    let window_title = format!("Chip - {}", &args[1]);
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title(window_title.as_str())
        .build();
    rl.set_target_fps(60);

    // At 500hz, CPU should cycle every 2 milliseconds.
    // 1 sec / 500 times = 0.002 secs, or 2 ms
    let frequency_hz  = 500;
    let tick_interval = Duration::from_secs(1) / frequency_hz;

    // --- Main window loop -----------------------------------------------------
    while !rl.window_should_close() {
        // We assign the variable d to represent the active drawing context
        let mut d = rl.begin_drawing(&thread);

        let mut next_tick = Instant::now();
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

        d.clear_background(Color::BLACK);
        d.draw_fps(0, 0);

        // Height
        for h in 0..32 {
            // Width
            for w in 0..64 {
                let pixel: i32 = (h * 64) + w;

                if chip8.display[pixel as usize] == true {
                    d.draw_rectangle(
                        w * SCALE,
                        h * SCALE,
                        SCALE,
                        SCALE,
                        Color::WHITE);
                }
            }
        }
    }
    // --------------------------------------------------------------------------



    // TODO:
    // - Display
    // - Master clock
    //   - Calling cpu.tick_timers() at 60Hz
    // - Keyboard input
    // - Audio output
}
