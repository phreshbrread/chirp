// NOTE: Input & sound are currently broken due to changes in cycle logic

use raylib::prelude::*;
use std::sync::mpsc;
use std::{env, sync::Arc, thread, time::Duration};

// Include cpu.rs and timers.rs
mod chip_timer;
mod cpu;

use chip_timer::ChipTimer;
use chirp::*;

// Resources:
//   - https://austinmorlan.com/posts/chip8_emulator/
//   - https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
//   - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Annn
//   - https://wiki.xxiivv.com/site/chip8.html
//   - https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

fn main() {
    // --- Flags ------------------------------------------------
    let original_behaviour = false;
    let rom_str: Box<str>;

    // Handle argument stuff in its own scope so it can all be freed when we're done
    {
        let mut flags: Vec<Flag> = vec![Flag::new(
            "-o",
            "--original",
            "Emulates original hardware behaviour",
            &original_behaviour,
        )];

        // Ensure arg is given
        let mut argv: Vec<String> = env::args().collect();
        let argc = argv.len();

        // First argument is always binary path
        if argc < 2 {
            show_help(flags);
        }

        // Iterate through args and match to corresponding flag
        let mut activated_flags = 0;
        for arg in 1..argc {
            for flag in 0..flags.len() {
                if *argv[arg] == *flags[flag].short || *argv[arg] == *flags[flag].long {
                    flags[flag].active = &true;
                    activated_flags += 1;
                }
            }
        }

        if activated_flags != (argc - 2) {
            show_help(flags);
        }

        rom_str = argv.pop().unwrap().into_boxed_str();
    }

    // ----------------------------------------------------------

    let mut chip8: cpu::Chip8 = cpu::Chip8::new(original_behaviour);
    println!("Initialised CPU");

    // Attempt to load ROM
    chip8.load_rom(&rom_str);

    let (mut display_tx, display_rx) = mpsc::channel();

    // Initialize global timer handle
    let timer_handle = Arc::new(ChipTimer::new());

    let timer_clone = Arc::clone(&timer_handle);
    let _timer_thread = thread::spawn(move || {
        // Timer updates at a fixed 60Hz
        let interval = Duration::from_secs(1) / 60;

        loop {
            timer_clone.tick();
            thread::sleep(interval);
        }
    });

    // --- Raylib init --------------------------------
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title(format!("Chirp | {}", rom_str).as_str())
        .build();
    rl.set_target_fps(60);
    // ------------------------------------------------

    let audio_handle = RaylibAudio::init_audio_device().expect("Failed to initialise audio device");
    let beep = audio_handle
        .new_sound("assets/beep.wav")
        .expect("Failed to load beep sound file");

    let _cycle_thread = thread::spawn(move || {
        let interval = Duration::from_secs(1) / 500;

        loop {
            // TODO: Fix input
            //chip8.keypad = poll_input(&d);
            chip8.cycle(Arc::clone(&timer_handle), &display_tx);

            // TODO: Account for drift
            thread::sleep(interval);
        }
    });

    // Set screen to blank
    let mut screen = [false; CHIP8_DISPLAY_SIZE];

    // Main window loop
    while !rl.window_should_close() {
        // We assign the variable d to represent the active drawing context
        let mut d = rl.begin_drawing(&thread);

        // TODO: Fix sound
        // if timer_handle.should_beep() {
        //     beep.play();
        // }

        d.clear_background(Color::BLACK);

        // Only update display array if it changes
        screen = match display_rx.try_recv() {
            Err(_) => screen,
            Ok(o) => o,
        };

        for h in 0..32 {
            // Height
            for w in 0..64 {
                // Width
                let pixel: i32 = (h * 64) + w;

                if screen[pixel as usize] == true {
                    d.draw_rectangle(
                        w * SCREEN_SCALE,
                        h * SCREEN_SCALE,
                        SCREEN_SCALE,
                        SCREEN_SCALE,
                        Color::WHITE,
                    );
                }
            }
        }

        d.draw_fps(0, 0);
    }
}
