use raylib::{
    RaylibHandle,
    prelude::{KeyboardKey, RaylibDrawHandle},
};
use std::process;

pub const START_ADDRESS: u16 = 0x200; // First 512 bytes reserved for system
pub const FONTSET_SIZE: usize = 80; // Fonts only take up 80 bytes
pub const MAX_ROM_SIZE: usize = 4096 - 512;

// TODO: Allow for adjustment
pub const SCREEN_SCALE: i32 = 20;
pub const SCREEN_W: i32 = 64 * SCREEN_SCALE;
pub const SCREEN_H: i32 = 32 * SCREEN_SCALE;

pub const CHIP8_KEYS: [KeyboardKey; 16] = [
    KeyboardKey::KEY_X,     // 0
    KeyboardKey::KEY_ONE,   // 1
    KeyboardKey::KEY_TWO,   // 2
    KeyboardKey::KEY_THREE, // 3
    KeyboardKey::KEY_Q,     // 4
    KeyboardKey::KEY_W,     // 5
    KeyboardKey::KEY_E,     // 6
    KeyboardKey::KEY_A,     // 7
    KeyboardKey::KEY_S,     // 8
    KeyboardKey::KEY_D,     // 9
    KeyboardKey::KEY_Z,     // A (10)
    KeyboardKey::KEY_C,     // B (11)
    KeyboardKey::KEY_FOUR,  // C (12)
    KeyboardKey::KEY_R,     // D (13)
    KeyboardKey::KEY_F,     // E (14)
    KeyboardKey::KEY_V,     // F (15)
];

pub const CHIP8_DISPLAY_SIZE: usize = 64 * 32;
pub type DisplayArray = [bool; CHIP8_DISPLAY_SIZE];
pub type KeypadArray = [bool; 16];

#[derive(Debug)]
pub struct Flag<'a> {
    pub short: Box<str>,
    pub long: Box<str>,
    pub desc: Box<str>,
    pub active: &'a bool,
}

impl<'a> Flag<'a> {
    pub fn new(s: &str, l: &str, d: &str, a: &'a bool) -> Self {
        return Self {
            short: s.into(),
            long: l.into(),
            desc: d.into(),
            active: a,
        };
    }
}

pub fn poll_input_new(d: &RaylibHandle) -> KeypadArray {
    let mut keypad = [false; 16];

    for (index, &key) in CHIP8_KEYS.iter().enumerate() {
        keypad[index] = d.is_key_down(key);
    }

    return keypad;
}

pub fn poll_input(d: &RaylibDrawHandle) -> KeypadArray {
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
