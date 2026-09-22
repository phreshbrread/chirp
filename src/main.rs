use raylib::prelude::*;
use std::{
   path::PathBuf, sync::{Arc, RwLock, mpsc}, thread, time::Duration
};
use clap::Parser;

// Include cpu.rs and timers.rs
mod chip_timer;
mod cpu;
mod execute;

use chip_timer::ChipTimer;
use chirp::*;

// Resources:
//   - https://austinmorlan.com/posts/chip8_emulator/
//   - https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
//   - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Annn
//   - https://wiki.xxiivv.com/site/chip8.html
//   - https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

#[derive(Parser, Debug)]
struct Args {
    //#[arg(short, long)]
    //system: Option<String>,

    /// Emulate original hardware behaviour
    #[arg(short, long)]
    original_behaviour: bool,

    /// Display current framerate
    #[arg(short = 'f', long)]
    show_fps: bool,

    /// Enable using the ESC key to exit
    #[arg(short, long)]
    esc_quits: bool,

    /// Path to a CHIP-8 ROM file
    rom_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    dbg!(&args);

    let shared_framebuffer = Arc::new(RwLock::new([false; CHIP8_DISPLAY_SIZE]));

    let mut chip8: cpu::Chip8 = cpu::Chip8::new(args.original_behaviour);
    println!("Initialised CPU");

    // Attempt to load ROM
    chip8.load_rom(&args.rom_path);

    // Set up channels
    let (keypad_tx, keypad_rx) = mpsc::channel();

    // Initialize global timer handle
    let timer_handle = Arc::new(ChipTimer::new());

    let timer_clone = Arc::clone(&timer_handle);
    let _timer_thread = thread::spawn(move || {
        // Timer updates at a fixed 60Hz
        let interval = Duration::from_secs(1) / 60;

        precise_interval_loop(interval, || {
            timer_clone.tick();
        });
    });

    let window_title = format!("Chirp | {}", args.rom_path.to_str().unwrap());

    // Initialize window
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_W, SCREEN_H)
        .title(window_title.as_str())
        .build();
    rl.set_target_fps(60);

    // Disable ESC to quit
    if !args.esc_quits {
        rl.set_exit_key(None);
    }

    let audio_handle = RaylibAudio::init_audio_device().expect("Failed to initialise audio device");
    audio_handle.set_audio_stream_buffer_size_default(4096);

    let beep = audio_handle
        .new_sound("assets/beep.wav")
        .expect("Failed to load beep sound file");

    let timer_clone = Arc::clone(&timer_handle);
    let cycle_framebuffer = Arc::clone(&shared_framebuffer);
    let _cycle_thread = thread::spawn(move || {
        let interval = Duration::from_secs(1) / 500;

        precise_interval_loop(interval, || {
            chip8.cycle(&timer_clone, &cycle_framebuffer, &keypad_rx);
        });
    });

    // Initialise blank screen
    let mut screen = [false; CHIP8_DISPLAY_SIZE];

    // Main window loop
    while !rl.window_should_close() {
        // We assign the variable d to represent the active drawing context
        let mut d = rl.begin_drawing(&thread);

        // Send input first
        keypad_tx.send(poll_input(&d)).unwrap();

        if timer_handle.should_beep() {
            beep.play();
        }

        d.clear_background(Color::BLACK);

        // Only update display array if it changes
        screen = match shared_framebuffer.try_read() {
            Ok(o) => *o,
            Err(_) => screen,
        };

        // Draw pixels row by row
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

        if args.show_fps {
            d.draw_fps(0, 0);
        }
    }
}
