mod cpu;
mod execute;

use chirp::*;
use clap::Parser;
use minifb::{Key, Window, WindowOptions};
use rodio::source::{SineWave, Source};
use std::{
    sync::atomic::Ordering,
    thread,
    time::{Duration, Instant},
};

fn main() {
    let args = Args::parse();
    dbg!(&args);

    // Initialize chip system & framebuffer
    let mut chip8 = cpu::Chip8::new(&args.sys_type.unwrap_or_default());

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

    // Main window loop
    while window.is_open() {
        let frame_start = Instant::now();

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

        if args.show_fps {
            // TODO: Show FPS
        }

        // Update window
        window
            .update_with_buffer(&chip8.framebuffer, SCREEN_W, SCREEN_H)
            .unwrap();

        let frame_end = Instant::now();

        let _frametime = frame_end - frame_start;
        //println!("Frametime: {:#?}", frametime);
    }
}
