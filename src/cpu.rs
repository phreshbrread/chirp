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
    pub stack:       Vec<u16>,       // Call stack - list of memory addresses to keep track of subroutines
    pub delay_timer: u8,             // Count down to 0 at 60Hz, independent of CPU clock speed
    pub sound_timer: u8,             // Same as delay_timer, but the system emits a beep if value > 0
    pub keypad:      [bool; 16],     // 16 keys, either pressed or not pressed
    pub display:     [bool; 64 * 32] // 64 x 32 monochrome display, each pixel either on or off
}

impl Chip8 {
    pub fn new() -> Self {
        let mut new_cpu = Self {
            memory:      [0; 4096],              // Clear memory
            registers:   [0; 16],                // Clear registers
            index_reg:   0,                      // Clear index register
            prog_ctr:    START_ADDRESS,          // Games start at 0x200 as the first 512 bytes are reserved for the system
            stack:       Vec::with_capacity(16), // Call stack can hold up to 16 addresses
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
        match rom_file.read_to_end(&mut tmp_buf) {
            // () here means we do nothing if the read is successful
            Ok(_) => (),
            Err(e) => {
                println!("Error occurred: {:?}", e);
                std::process::exit(1);
            }
        };

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

        // Program counter is already set to the start address (0x200) in cpu::Chip8::new()
    }

    pub fn cycle(&mut self) {
        // --- Fetch stage -------------------------------------------------
        // Fetch the next two bytes from the program counter
        // and combine them into a single 16-bit instruction.
        //
        // Since an opcode is 16-bit (two bytes), it means the first 8 bits
        // are on the right hand side of the container, so we shift them 8
        // bits to the left using "<< 8" and then use a bitwise OR (|) to
        // essentially append the next byte to form a full 16-bit opcode
        let opcode: u16 =
            ((self.memory[self.prog_ctr as usize] as u16) << 8) |
            (self.memory[self.prog_ctr as usize + 1]) as u16;

        // Increment program counter
        self.prog_ctr += 2;
        // -----------------------------------------------------------------

        // --- Decode ------------------------------------------------------
        // When fetching an opcode like 0x6A02, it comes as a raw, packed 16-bit chunk of binary
        // data. The CPU cannot simply execute it as-is, and instead must route different pieces
        // of that 16-bit number to different parts of its virtual circuitry.
        // To do that, we slice that 16-bit number into four individual 4-bit variables
        // (called nibbles, as they are half the size of a byte), and then recombining those
        // nibbles to form the system's core variables: X, Y, N, NN, and NNN.

        let n1:  u8    = ((opcode & 0xF000) >> 12) as u8;    // Primary opcode group identifier
        let x:   usize = ((opcode & 0x0F00) >> 8)  as usize; // Target register index
        let y:   usize = ((opcode & 0x00F0) >> 4)  as usize; // Secondary source register index
        let n:   u8    =  (opcode & 0x000F)        as u8;    // Immediate nibble
        let nn:  u8    =  (opcode & 0x00FF)        as u8;    // Immediate byte
        let nnn: u16   =   opcode & 0x0FFF;                  // Memory address

        // -----------------------------------------------------------------

        // --- Execute -----------------------------------------------------
        // Opcodes are grouped by their first nibble (n1). Some opcodes share the same n1 value,
        // so we can use n to identify instructions in the same group (nn in group 0x0's case)
        match n1 {
            0x0 => {
                match nn {
                    0xE0 => {
                        todo!("Clear screen");
                    },
                    0xEE => {
                        // 00EE: Set program counter to the last address on the stack,
                        // then pop said address
                        self.prog_ctr = self.stack.pop()
                            .expect("Failed to return from subroutine");
                        },
                    _ => panic!("Unknown 0x0 group opcode"),
                }
            },

            0x1 => {
                // 1NNN: Jump to nnn
                self.prog_ctr = nnn;
            },

            0x2 => {
                // 2NNN: Save / push the current program counter to the stack so we can
                // return later, then set the program counter to NNN
                self.stack.push(self.prog_ctr);
                self.prog_ctr = nnn;
            },

            0x3 => {
                // 3XNN: Skip next instruction if VX = nn
                if self.registers[x] == nn {
                    self.prog_ctr += 2;
                }
            },

            0x4 => {
                // 4XNN: Skip next instruction if VX != nn
                if self.registers[x] != nn {
                    self.prog_ctr += 2;
                }
            },

            0x5 => {
                // 5XY0: Skip next instruction if VX and VY are equal
                if self.registers[x] == self.registers[y] {
                    self.prog_ctr += 2;
                }
            },

            0x6 => {
                // 6XNN: Set register VX to value of NN
                self.registers[x] = nn;
            },

            0x7 => {
                // 7XNN: Add value of NN to VX
                // We use .wrapping_add() because this instruction on real
                // hardware wraps around when the value overflows,
                // otherwise we would crash here.
                self.registers[x] += self.registers[x].wrapping_add(nn);
            },

            // Maths engine
            0x8 => {
                match n {
                    0x0 => {
                        // 8XY0 - Set VX to value in VY
                        self.registers[x] = self.registers[y];
                    },
                    0x1 => {
                        // 8XY1: Set VX to bitwise OR of VX and VY
                        todo!("8XY1");
                    },
                    _ => todo!("{:06X}", nn),
                };
            },

            0x9 => {
                // 9XY0: Skip next instruction if VX and VY are NOT equal
                if self.registers[x] != self.registers[y] {
                    self.prog_ctr += 2;
                }
            },

            0xA => {
                // ANNN: Set value of index register to nnn
                self.index_reg = nnn;
            },

            0xB => {
                todo!("0xB group");
            },

            0xC => {
                todo!("0xC group");
            },

            // Display
            0xD => {
                // DXYN: Draw to screen and check collisions
                // Apply modulo to wrap sprites around edges if needed
                let start_x = self.registers[x] % 64;
                let start_y = self.registers[y] % 32;
                let height = n;
                let mut collision = false;

                // Vertical loop
                for i in 0..height {
                    let sprite_byte: u8 = self.memory[self.index_reg as usize + i as usize];
                    let row_coord = (start_y + i) % 32;

                    // Horizontal loop
                    for j in 0..8 {
                        let column = (start_x + j) % 64;

                        let isolated_bit = sprite_byte >> (7 - j) & 1;
                        let display_index: usize = row_coord as usize * 64 + column as usize;

                        if (isolated_bit) == 1 {
                            if self.display[display_index] == true {
                                collision = true;
                                self.display[display_index] = false;
                            } else {
                                self.display[display_index] = true;
                            }
                        }
                    }
                }

                if collision {
                    self.registers[15] = 1;
                } else {
                    self.registers[15] = 0;
                }
            },

            // Keypad checks
            0xE => {
                todo!("0xE group");
            },

            // Timers, memory and fonts
            0xF => {
                todo!("0xF group");
            },

            _ => {
                todo!("Implement operation {:#06x}", n1);
            },
        };

        // 00E0 (clear screen)
        // 1NNN (jump)
        // 6XNN (set register VX)
        // 7XNN (add value to register VX)
        // ANNN (set index register I)
        // DXYN (display/draw)

        // -----------------------------------------------------------------

        // --- Update state -----------------------------------------------------------
        // TODO: Update timers
        // ----------------------------------------------------------------------------



        // Chip-8 opcodes are 16 bits (2 bytes long).
        // Virtual memory only stores 8 bits (1 byte) at a time.
        // Each instruction is split between two adjacent memory slots.
        // Each instruction must be fetched by getting the first and second byte,
        // then combining them into a single two byte instruction.
    }
}
