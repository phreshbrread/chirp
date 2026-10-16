use minifb::{Window, WindowOptions};
use std::{
    sync::{Arc, RwLock, mpsc},
    thread,
    time::Duration,
};

mod chip_timer;
mod cli_args;
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

fn main() {
    let args = cli_args::parse_cli_args();
    dbg!(&args);

    // Initialize chip system & framebuffer
    let mut chip8 = cpu::Chip8::new(args.original_behaviour);
    let shared_framebuffer = Arc::new(RwLock::new([false; CHIP8_DISPLAY_SIZE]));

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

    // Initialize window
    let window_title = format!("Chirp | {}", args.rom_path.to_str().unwrap());

    let mut window = match Window::new(
        "Resizable Window",
        DISPLAY_W * SCREEN_SCALE,
        DISPLAY_H * SCREEN_SCALE,
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

    let timer_clone = Arc::clone(&timer_handle);
    let cycle_framebuffer = Arc::clone(&shared_framebuffer);
    let _cycle_thread = thread::spawn(move || {
        let interval = Duration::from_secs(1) / 500;

        precise_interval_loop(interval, || {
            chip8.cycle(&timer_clone, &cycle_framebuffer, &keypad_rx);
        });
    });

    // Initialise blank screen.
    let mut screen = [false; CHIP8_DISPLAY_SIZE];

    // Minifb expects a framebuffer of 32-bit unsigned integers.
    let mut buffer: Vec<u32> = vec![0 as u32; DISPLAY_W * DISPLAY_H];

    // Main window loop
    while window.is_open() {
        if args.esc_quits && window.is_key_down(minifb::Key::Escape) {
            std::process::exit(0);
        }

        // Send input before rendering anything.
        keypad_tx
            .send(poll_input(&window))
            .expect("Failed to poll input.");

        if timer_handle.should_beep() {
            // TODO: Play beep
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

        // Update window
        window
            .update_with_buffer(&buffer, DISPLAY_W, DISPLAY_H)
            .unwrap();
    }

    // TODO: Show FPS
}
