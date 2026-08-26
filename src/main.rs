use raylib::prelude::*;
use std::{env, sync::Arc, thread, time::Duration};

use chirp::*;

// Include cpu.rs and timers.rs
mod cpu;
mod timers;

// Resources:
//   - https://austinmorlan.com/posts/chip8_emulator/
//   - https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
//   - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Annn
//   - https://wiki.xxiivv.com/site/chip8.html
//   - https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

fn main() {
    // --- Flags ------------------------------------------------
    let original_behaviour = false;
    let mut flags: Vec<Flag> = vec![Flag {
        short: "-o".to_owned(),
        long: "--original".to_owned(),
        desc: "Emulates original behaviour".to_owned(),
        active: &original_behaviour,
    }];

    // Ensure arg is given
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    if argc < 2 {
        show_help(flags);
    }

    let mut activated_flags = 0;

    for i in 1..argc {
        // For each argument
        for j in 0..flags.len() {
            // For each valid flag
            if argv[i] == flags[j].short || argv[i] == flags[j].long {
                flags[j].active = &true;
                activated_flags += 1;
            }
        }
    }

    if activated_flags != (argc - 2) {
        show_help(flags);
    }
    // ----------------------------------------------------------

    let mut chip8: cpu::Chip8 = cpu::Chip8::new(original_behaviour);
    println!("Initialised CPU");

    // Attempt to load ROM from first argument
    chip8.load_rom(&argv[argc - 1]);

    // Initialize global timer handle
    let timer_handle = Arc::new(timers::ChipTimer::new());

    //let arc_chip8 = Arc::new(&mut chip8);

    // --- Timer thread ----------------------------------------------------------
    let timer_clone = Arc::clone(&timer_handle);
    thread::spawn(move || {
        // Timer updates at a fixed 60Hz
        let interval = Duration::from_secs(1) / 60;

        loop {
            timer_clone.tick();
            thread::sleep(interval);
        }
    });
    // ---------------------------------------------------------------------------

    // Raylib init
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title(format!("Chirp - {}", &argv[1]).as_str())
        .build();
    rl.set_target_fps(500);

    let audio_handle = RaylibAudio::init_audio_device().expect("Failed to initialise audio device");
    let beep = audio_handle
        .new_sound("assets/beep.wav")
        .expect("Failed to load beep sound file");

    // Main window loop
    while !rl.window_should_close() {
        // We assign the variable d to represent the active drawing context
        let mut d = rl.begin_drawing(&thread);

        chip8.keypad = poll_input(&d);

        // CPU should run at 500Hz
        // TODO: Run in a seperate thread
        chip8.cycle(Arc::clone(&timer_handle));

        if timer_handle.should_beep() {
            beep.play();
        }

        // --- Draw pixels ------------------------------------------------------
        d.clear_background(Color::BLACK);

        for h in 0..32 {
            // Height
            for w in 0..64 {
                // Width
                let pixel: i32 = (h * 64) + w;

                if chip8.display[pixel as usize] == true {
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
        // ----------------------------------------------------------------------
    }
}
