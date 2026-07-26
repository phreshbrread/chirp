// Chip-8 CPU

use std::io::Read;
use std::fs::File;

const START_ADDRESS: u16   = 0x200; // First 512 bytes reserved for system
const END_ADDRESS:   u16   = 0xFFF;
const FONTSET_SIZE:  usize = 80;    // Fonts only take up 80 bytes

#[derive(Debug)]
pub struct Chip8 {
    pub memory:      [u8; 4096],     // 4KB of RAM (u8 is one byte)
    pub registers:   [u8; 16],       // Chip-8 has 16 registers, V0 - V9, and VA - VF
    pub index_reg:   u16,            // 16-bit register to hold memory addresses
    pub prog_ctr:    u16,            // Program counter
    pub stack:       [u16; 16],      // Call stack - list of memory addresses to keep track of subroutines
    pub stack_ptr:   u16,            // Stack pointer - tracks the top of the call stack
    pub delay_timer: u8,             // Count down to 0 at 60Hz, independent of CPU clock speed
    pub sound_timer: u8,             // Same as delay_timer, but the system emits a beep if value > 0
    pub keypad:      [bool; 16],     // 16 keys, either pressed or not pressed
    pub display:     [bool; 64 * 32] // 64 x 32 monochrome display, each pixel either on or off
}

impl Chip8 {
    pub fn new() -> Self {
        let mut new_cpu = Self {
            memory:      [0; 4096],     // Clear memory
            registers:   [0; 16],       // Clear registers
            index_reg:   0,             // Clear index register
            prog_ctr:    START_ADDRESS, // Games start at 0x200 as the first 512 bytes are reserved for the system
            stack:       [0; 16],
            stack_ptr:   0,
            delay_timer: 0,
            sound_timer: 0,
            keypad:      [false; 16],     // Set all keys to unpressed
            display:     [false; 64 * 32] // Set all pixels to black (off)
        };

        new_cpu.load_font();

        return new_cpu;
    }

    fn load_font(&mut self) {
        // Each character is 5 bytes tall
        const FONTSET: [u8; FONTSET_SIZE] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        // Put characters into first 80 bytes of memory
        self.memory[0..FONTSET_SIZE].copy_from_slice(&FONTSET);
        println!("Loaded font set");
    }

    pub fn load_rom(&mut self, rp: &str) {
        let max_rom_size = 4096 - 512;

        // Let open() consume "rp" since we won't need it again
        let mut rom_file = match File::open(rp) {
            Ok(data) => data,
            Err(error) => {
                println!("Failed to open ROM: {}", error);
                std::process::exit(1);
            },
        };

        // Read ROM contents into temporary buffer
        let mut tmp_buf = Vec::new();
        rom_file.read_to_end(&mut tmp_buf);

        // Check ROM size
        if tmp_buf.len() > max_rom_size {
            println!("Invalid ROM file: ROM is too large");
            std::process::exit(1);
        }
        println!("Opened ROM of size {} bytes", tmp_buf.len());

        let start_address: usize = START_ADDRESS as usize;

        // tmp_buf.iter().enumerate() returns a tuple - index and the byte being read
        for (i, &byte) in tmp_buf.iter().enumerate() {
            // Starting from 0x200, replace each byte in Chip-8
            // memory with the corresponding byte from the ROM
            self.memory[start_address + i] = byte;
        }
        println!("Loaded ROM into Chip-8 memory");

        // Program counter is already set to the start address (0x200)
    }

    pub fn cycle(&mut self) {
        // --- Fetch -------------------------------------------------------
        // Fetch the next two bytes from the program counter
        // and combine them into a single 16-bit instruction.
        // The "<<" operator shifts the bits left
        let mut opcode =
            self.memory[self.prog_ctr as usize] <<
            self.memory[self.prog_ctr as usize + 1];

        // Increment program counter
        self.prog_ctr += 2;
        // -----------------------------------------------------------------

        // --- Decode ------------------------------------------------------
        // TODO: Decode the instruction to determine
        // which operation needs to occur.
        // -----------------------------------------------------------------

        // --- Execute -----------------------------------------------------
        // TODO: Execute the instruction.
        // -----------------------------------------------------------------

        // ----------------------------------------------------------------------------
        // Chip-8 opcodes are 16 bits (2 bytes long).
        // Virtual memory only stores 8 bits (1 byte) at a time.
        // Each instruction is split between two adjacent memory slots.
        // Each instruction must be fetched by getting the first and second byte,
        // then combining them into a single 2 byte instruction.
    }
}
