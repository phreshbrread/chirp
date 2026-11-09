use minifb::{Key, Window, WindowOptions};
use rodio::{
    MixerDeviceSink, Player,
    source::{SineWave, Source},
};
use std::{
    sync::{Arc, RwLock, atomic::Ordering, mpsc},
    thread,
    time::Duration,
};

mod cli_args;
mod cpu;
mod execute;

use chirp::*;

// Resources:
//   - https://austinmorlan.com/posts/chip8_emulator/
//   - https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
//   - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Annn
//   - https://wiki.xxiivv.com/site/chip8.html
//   - https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

fn main() {
    let args = cli_args::parse_cli_args();
    dbg!(&args);

    // Initialize chip system & framebuffer
    let mut chip8 = cpu::Chip8::new(args.original_behaviour);
    let shared_framebuffer = Arc::new(RwLock::new([false; CHIP8_DISPLAY_SIZE]));

    // Attempt to load ROM
    chip8.load_rom(&args.rom_path);

    // Set up IO channels
    let (keypad_tx, keypad_rx) = mpsc::channel();

    // Global timer thread
    let _timer_thread = thread::spawn(move || {
        // Timer updates at a fixed 60Hz
        let interval = Duration::from_secs(1) / 60;

        precise_interval_loop(interval, || {
            tick_timers();
        });
    });

    // Initialize window
    let window_title = format!("Chirp | {}", args.rom_path.to_str().unwrap());

    let mut window = match Window::new(
        "Resizable Window",
        SCREEN_W * SCREEN_SCALE,
        SCREEN_H * SCREEN_SCALE,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("Failed to create minifb window: {err}");
            std::process::exit(1);
        }
    };
    window.set_target_fps(60);
    window.set_title(&window_title);

    // TODO: Replace this
    //let beep = audio_handle
    //    .new_sound("assets/beep.wav")
    //    .expect("Failed to load beep sound file");

    // Get audio handle
    // TODO: Allow reusing beep sound
    let audio_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("Failed to open default audio stream");
    let beep = rodio::Player::connect_new(&audio_handle.mixer());
    beep.append(SineWave::new(440.0).amplify(0.50));

    let cycle_framebuffer = Arc::clone(&shared_framebuffer);
    let _cycle_thread = thread::spawn(move || {
        let interval = Duration::from_secs(1) / 500;

        precise_interval_loop(interval, || {
            chip8.cycle(&cycle_framebuffer, &keypad_rx);
        });
    });

    // Initialise blank screen.
    let mut screen = [false; CHIP8_DISPLAY_SIZE];

    // Minifb expects a framebuffer of 32-bit unsigned integers.
    let mut buffer: Vec<u32> = vec![0; SCREEN_W * SCREEN_H];

    // Main window loop
    while window.is_open() {
        if args.esc_quits && window.is_key_down(Key::Escape) {
            std::process::exit(0);
        }

        // Send input before rendering anything.
        keypad_tx
            .send(poll_input(&window))
            .expect("Failed to poll input.");

        // Play beep when sound timer > 0
        if SOUND_TIMER.load(Ordering::Relaxed) > 0 {
            // TODO: Suppress harsh start/stop noise
            beep.play();
        } else if !beep.is_paused() {
            beep.pause();
        }

        // Only update screen array if the framebuffer changes.
        if let Ok(new_screen) = shared_framebuffer.try_read() {
            screen = *new_screen;
        }

        // Convert the Chip-8 true/false array into 32-bit hex values for minifb.
        for (i, e) in buffer.iter_mut().enumerate() {
            // White if true, black if false
            *e = if screen[i] { 0xFFFFFF } else { 0x000000 };
        }

        if args.show_fps {
            // TODO: Show FPS

            // for i in &mut buffer[1..100] {
            //     *i = 0x00FF00;
            // }
        }

        // Update window
        window
            .update_with_buffer(&buffer, SCREEN_W, SCREEN_H)
            .unwrap();
    }
}
