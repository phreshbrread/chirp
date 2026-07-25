// Chip-8 CPU

#[derive(Debug)]
pub struct Chip8 {
    pub memory:      [u8; 4096],     // 4KB of RAM (u8 is one byte)
    pub registers:   [u8; 16],       // Chip-8 has 16 registers, V0 - V9, and VA - VF
    pub index_reg:   u16,            // 16-bit register to hold memory addresses
    pub p_counter:   u16,            // Program counter
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
            memory:      [0; 4096],       // Clear memory
            registers:   [0; 16],         // Clear registers
            index_reg:   0,
            p_counter:   0x200,           // Games start at 0x200 as the first 512 bytes are reserved for the system
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
        const FONTSET: [u8; 80] = [
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
        self.memory[0..80].copy_from_slice(&FONTSET);
        println!("Loaded font set")
    }
}

