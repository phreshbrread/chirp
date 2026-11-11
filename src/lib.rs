mod cpu;
mod execute;

use bitvec::prelude::*;
use clap::{Parser, ValueEnum};
use minifb::{Key, Window};
use std::{
    path::PathBuf,
    sync::{
        Arc, RwLock,
        atomic::{AtomicU8, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

pub static DELAY_TIMER: AtomicU8 = AtomicU8::new(0);
pub static SOUND_TIMER: AtomicU8 = AtomicU8::new(0);

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

// TODO: Replace display with BitArr
pub const CHIP8_DISPLAY_SIZE: usize = 64 * 32;
pub type DisplayArray = [bool; CHIP8_DISPLAY_SIZE];
pub type Keypad = BitArr!(for 16);
pub type Framebuffer = Vec<u32>;

pub fn tick_timers() {
    if DELAY_TIMER.load(Ordering::Relaxed) > 0 {
        DELAY_TIMER.fetch_sub(1, Ordering::Relaxed);
    }
    if SOUND_TIMER.load(Ordering::Relaxed) > 0 {
        SOUND_TIMER.fetch_sub(1, Ordering::Relaxed);
    }
}

pub fn convert_framebuffer(chip_display: &DisplayArray) -> Framebuffer {
    let mut buffer: Framebuffer = vec![0; SCREEN_W * SCREEN_H];

    for (i, e) in buffer.iter_mut().enumerate() {
        // White if true, black if false
        *e = if chip_display[i] { 0xFFFFFF } else { 0x000000 };
    }

    return buffer;
}

pub fn update_framebuffer(cycle_framebuffer: &Arc<RwLock<DisplayArray>>, display: &DisplayArray) {
    let mut lock = cycle_framebuffer.write().unwrap();
    *lock = *display;
}

pub fn poll_input(w: &Window) -> Keypad {
    let mut keypad: Keypad = BitArray::ZERO;

    for (index, &key) in CHIP8_KEYS.iter().enumerate() {
        keypad.set(index, w.is_key_down(key));
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

#[derive(Debug, Clone, PartialEq, ValueEnum, Default)]
pub enum SystemType {
    #[default]
    Chip,
    SuperChip,
    XoChip,
}

#[derive(Parser, Debug)]
pub struct Args {
    /// Variant to emulate
    #[arg(short, long)]
    pub sys_type: Option<SystemType>,

    /// Display current framerate
    #[arg(short = 'f', long)]
    pub show_fps: bool,

    /// Enable using the ESC key to exit
    #[arg(short, long)]
    pub esc_quits: bool,

    /// Path to a CHIP-8 ROM file
    pub rom_path: PathBuf,
}
