use raylib::prelude::{KeyboardKey, RaylibDrawHandle};
use std::process;

pub const START_ADDRESS: u16 = 0x200; // First 512 bytes reserved for system
pub const FONTSET_SIZE: usize = 80; // Fonts only take up 80 bytes
pub const MAX_ROM_SIZE: usize = 4096 - 512;

// TODO: Allow for adjustment
pub const SCREEN_SCALE: i32 = 20;
pub const SCREEN_W: i32 = 64 * SCREEN_SCALE;
pub const SCREEN_H: i32 = 32 * SCREEN_SCALE;

pub const CHIP8_KEYS: [KeyboardKey; 16] = [
    KeyboardKey::KEY_X,
    KeyboardKey::KEY_ONE,
    KeyboardKey::KEY_TWO,
    KeyboardKey::KEY_THREE,
    KeyboardKey::KEY_Q,
    KeyboardKey::KEY_W,
    KeyboardKey::KEY_E,
    KeyboardKey::KEY_A,
    KeyboardKey::KEY_S,
    KeyboardKey::KEY_D,
    KeyboardKey::KEY_Z,
    KeyboardKey::KEY_C,
    KeyboardKey::KEY_FOUR,
    KeyboardKey::KEY_R,
    KeyboardKey::KEY_F,
    KeyboardKey::KEY_V,
];

#[derive(Debug)]
pub struct Flag<'a> {
    pub short: String,
    pub long: String,
    pub desc: String,
    pub active: &'a bool,
}

pub fn poll_input(d: &RaylibDrawHandle) -> [bool; 16] {
    let mut keypad = [false; 16];

    for (index, &key) in CHIP8_KEYS.iter().enumerate() {
        keypad[index] = d.is_key_down(key);
    }

    return keypad;
}

pub fn show_help(f: Vec<Flag>) -> ! {
    println!("USAGE:\n chirp [FLAGS] [ROM]\n");
    println!("FLAGS:");

    for flag in f.iter() {
        println!("  {:<5}{:<15}{:}", flag.short, flag.long, flag.desc);
    }

    process::exit(1);
}
