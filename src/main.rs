use minifb::{Key, Window, WindowOptions};
use rodio::source::{SineWave, Source};
use std::{sync::atomic::Ordering, thread, time::Duration};

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

    // Attempt to load ROM
    chip8.load_rom(&args.rom_path);

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
        &window_title,
        SCREEN_W * SCREEN_SCALE,
        SCREEN_H * SCREEN_SCALE,
        WindowOptions {
            resize: true,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
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

    // Initialize beep
    let audio_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("Failed to open default audio stream");
    let beep = rodio::Player::connect_new(&audio_handle.mixer());
    beep.append(SineWave::new(440.0).amplify(0.50));

    // Minifb expects a framebuffer of 32-bit unsigned integers.
    let mut buffer: Vec<u32> = vec![0; SCREEN_W * SCREEN_H];

    // Main window loop
    while window.is_open() {
        if args.esc_quits && window.is_key_down(Key::Escape) {
            std::process::exit(0);
        }

        // Poll input before rendering anything.
        chip8.keypad = poll_input(&window);

        // Run 9 cycles per thread
        // 60hz * 9 cycles = 540hz
        for _ in 0..=9 {
            chip8.cycle();
        }

        // Play beep when sound timer > 0
        let should_beep = SOUND_TIMER.load(Ordering::Relaxed) > 0;
        if should_beep && beep.is_paused() {
            // TODO: Suppress harsh start/stop noise
            beep.play();
        } else if !should_beep && !beep.is_paused() {
            beep.pause();
        }

        // Convert the Chip-8 true/false array into 32-bit hex values for minifb.
        // TODO: Replace inefficient conversion with a better method
        for (i, e) in buffer.iter_mut().enumerate() {
            // White if true, black if false
            *e = if chip8.display[i] { 0xFFFFFF } else { 0x000000 };
        }

        if args.show_fps {
            // TODO: Show FPS
        }

        // Update window
        window
            .update_with_buffer(&buffer, SCREEN_W, SCREEN_H)
            .unwrap();
    }
}
