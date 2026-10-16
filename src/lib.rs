use minifb::{Key, Window};
use std::{
    thread,
    time::{Duration, Instant},
};

pub const START_ADDRESS: u16 = 0x200; // First 512 bytes reserved for system
pub const FONTSET_SIZE: usize = 80; // Fonts only take up 80 bytes
pub const MAX_ROM_SIZE: usize = 4096 - 512;

// TODO: Allow for adjustment
pub const SCREEN_SCALE: usize = 20;
pub const SCREEN_W: usize = 64;
pub const SCREEN_H: usize = 32;

pub const CHIP8_KEYS: [Key; 16] = [
    Key::X,    // 0
    Key::Key1, // 1
    Key::Key2, // 2
    Key::Key3, // 3
    Key::Q,    // 4
    Key::W,    // 5
    Key::E,    // 6
    Key::A,    // 7
    Key::S,    // 8
    Key::D,    // 9
    Key::Z,    // A (10)
    Key::C,    // B (11)
    Key::Key4, // C (12)
    Key::R,    // D (13)
    Key::F,    // E (14)
    Key::V,    // F (15)
];

pub const CHIP8_DISPLAY_SIZE: usize = 64 * 32;
pub type DisplayArray = [bool; CHIP8_DISPLAY_SIZE];
pub type KeypadArray = [bool; 16];
pub struct ChipFramebuffer(Vec<u32>);

pub fn poll_input(w: &Window) -> KeypadArray {
    let mut keypad = [false; 16];

    for (index, &key) in CHIP8_KEYS.iter().enumerate() {
        keypad[index] = w.is_key_down(key);
    }

    return keypad;
}

pub fn precise_interval_loop<F>(interval: Duration, mut task: F)
where
    F: FnMut(),
{
    let mut next_tick = Instant::now();

    loop {
        next_tick += interval;

        // Run closure given to this function
        task();

        let now = Instant::now();
        if now < next_tick {
            thread::sleep(next_tick - now);
        } else {
            next_tick = now;
        }
    }
}
