use std::{env, process};
use std::time::{Instant, Duration};
use raylib::prelude::*;

// Include cpu.rs
mod cpu;
mod cpu_new;
mod timers;

// Resources:
//   - https://austinmorlan.com/posts/chip8_emulator/
//   - https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
//   - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Annn
//   - https://wiki.xxiivv.com/site/chip8.html
//   - https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

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

    // Initialize global timer handle
    // TODO: Share this to functions properly
    let timer_handle = timers::ChipTimer::new();

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title(format!("Chip - {}", &args[1]).as_str())
        .build();
    rl.set_target_fps(120);

    // --- Main window loop -----------------------------------------------------
    while !rl.window_should_close() {
        // We assign the variable d to represent the active drawing context
        let mut d = rl.begin_drawing(&thread);

        // --- Keypad input -----------------------------------------------------
        // Keys are in order from 1 - 9, 0, then A to F
        chip8.keypad[1]  = d.is_key_down(KeyboardKey::KEY_ONE);
        chip8.keypad[2]  = d.is_key_down(KeyboardKey::KEY_TWO);
        chip8.keypad[3]  = d.is_key_down(KeyboardKey::KEY_THREE);
        chip8.keypad[12] = d.is_key_down(KeyboardKey::KEY_FOUR);

        chip8.keypad[4]  = d.is_key_down(KeyboardKey::KEY_Q);
        chip8.keypad[5]  = d.is_key_down(KeyboardKey::KEY_W);
        chip8.keypad[6]  = d.is_key_down(KeyboardKey::KEY_E);
        chip8.keypad[13] = d.is_key_down(KeyboardKey::KEY_R);

        chip8.keypad[7]  = d.is_key_down(KeyboardKey::KEY_A);
        chip8.keypad[8]  = d.is_key_down(KeyboardKey::KEY_S);
        chip8.keypad[9]  = d.is_key_down(KeyboardKey::KEY_D);
        chip8.keypad[14] = d.is_key_down(KeyboardKey::KEY_F);

        chip8.keypad[10] = d.is_key_down(KeyboardKey::KEY_Z);
        chip8.keypad[0]  = d.is_key_down(KeyboardKey::KEY_X);
        chip8.keypad[11] = d.is_key_down(KeyboardKey::KEY_C);
        chip8.keypad[15] = d.is_key_down(KeyboardKey::KEY_V);
        // ----------------------------------------------------------------------

        // TODO: Sound

        // --- Draw pixels ------------------------------------------------------
        d.clear_background(Color::BLACK);
        d.draw_fps(0, 0);

        for h in 0..32 {     // Height
            for w in 0..64 { // Width
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
        // ----------------------------------------------------------------------
    }
    // --------------------------------------------------------------------------
}
