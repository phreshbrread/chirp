// Chip-8 CPU

use std::io::Read;
use std::fs::File;

const START_ADDRESS: u16   = 0x200; // First 512 bytes reserved for system
const FONTSET_SIZE:  usize = 80;    // Fonts only take up 80 bytes

#[derive(Debug)]
pub struct Chip8 {
    pub memory:      [u8; 4096],      // 4KB of RAM (u8 is one byte)
    pub registers:   [u8; 16],        // Chip-8 has 16 registers, V0 - V9, and VA - VF
    pub index_reg:    u16,            // 16-bit register to hold memory addresses
    pub prog_ctr:     u16,            // Program counter
    pub stack:        Vec<u16>,       // Call stack - list of memory addresses to keep track of subroutines
    pub delay_timer:  u8,             // Count down to 0 at 60Hz, independent of CPU clock speed
    pub sound_timer:  u8,             // Same as delay_timer, but the system emits a beep if value > 0
    pub keypad:      [bool; 16],      // 16 keys, either pressed or not pressed
    pub display:     [bool; 64 * 32], // 64 x 32 monochrome display, each pixel either on or off
    pub og_behaviour: bool,           // Toggle to emulate quirks of original hardware
    pub should_beep:  bool,           // Flag to determine if beep sound should play
}

impl Chip8 {
    pub fn new() -> Self {
        let mut new_cpu = Self {
            memory:      [0; 4096],               // Clear memory
            registers:   [0; 16],                 // Clear registers
            index_reg:    0,                      // Clear index register
            prog_ctr:     START_ADDRESS,          // Games start at 0x200 as the first 512 bytes are reserved for the system
            stack:        Vec::with_capacity(16), // Call stack can hold up to 16 addresses
            delay_timer:  0,
            sound_timer:  0,
            keypad:      [false; 16],      // Set all keys to unpressed
            display:     [false; 64 * 32], // Turn all pixels off
            og_behaviour: false,           // Default to modern behaviour
            should_beep:  false,
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

    pub fn tick_timers(&mut self) {
        // Timer tick must run at 60hz independent of CPU cycles

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        // Beep should play if sound_timer is not zero
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            self.should_beep = true;
        } else {
            self.should_beep = false;
        }
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
        // so we can use n or nn to identify instructions in the same group
        // Groups using n: 0x8 and D
        // Groups using nn: 0x0, E, and F
        match n1 {
            0x0 => {
                match nn {
                    0xE0 => {
                        // 00E0: Clear the screen
                        self.display.fill(false);
                    },

                    0xEE => {
                        // 00EE: Set program counter to the last address on the stack,
                        // then pop said address
                        self.prog_ctr = self.stack.pop()
                            .expect("Failed to return from subroutine");
                        },
                    _ => unknown_opcode(opcode),
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
                self.registers[x] = self.registers[x].wrapping_add(nn);
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
                        self.registers[x] = self.registers[x] | self.registers[y];

                        // Reset VF to preserve original hardware quirk.
                        // This also occurs in 8XY2 and 8XY3.
                        if self.og_behaviour {
                            self.registers[15] = 0;
                        }

                    },

                    0x2 => {
                        // 8XY2: Set VX to bitwise AND of VX and VY
                        self.registers[x] = self.registers[x] & self.registers[y];

                        if self.og_behaviour {
                            self.registers[15] = 0;
                        }
                    },

                    0x3 => {
                        // 8XY3: Set VX to bitwise XOR of VX and VY
                        self.registers[x] = self.registers[x] ^ self.registers[y];

                        if self.og_behaviour {
                            self.registers[15] = 0;
                        }
                    },

                    0x4 => {
                        // 8XY4: Add value of VY to VX
                        // If the result is larger than 255, it will overflow VX, if this happens,
                        // we set the value of register VF to 1, otherwise, set it to 0.
                        // TODO: Improve code here

                        // First check if the result will overflow
                        let result: u16 = self.registers[x] as u16 + self.registers[y] as u16;

                        // Then we can perform the actual maths on VX
                        self.registers[x] = self.registers[x].wrapping_add(self.registers[y]);

                        // Finally, set VF accordingly
                        if result > 255 {
                            self.registers[15] = 1;
                        } else {
                            self.registers[15] = 0;
                        }
                    },

                    0x5 => {
                        // 8XY5: Subtract value of VY from VX

                        // Set VF to 1 if VX >= VY, saving into a temp variable to
                        // avoid overwriting the register for now
                        let borrow_flag: u8 = if self.registers[x] >= self.registers[y] {
                            1
                        } else {
                            0
                        };

                        // Save value into temp variable, wrapping result in the case of an
                        // underflow
                        let result = self.registers[x].wrapping_sub(self.registers[y]);

                        // We can now overwrite the actual registers
                        self.registers[x]  = result;
                        self.registers[15] = borrow_flag;
                    },

                    0x6 => {
                        // 8XY6: If the least significant bit of VX is 1, set VF
                        // to 1 (otherwise 0), then divide VX by 2.

                        // Original hardware would put the value of VY into VX first
                        if self.og_behaviour {
                            self.registers[x] = self.registers[y];
                        }

                        let tmp_vx = self.registers[x];

                        // To get the LSB we just mask VX with a bitwise AND 1
                        let lsb = tmp_vx & 1;

                        self.registers[15] = lsb;

                        // Divide the old value from VX by 2 by shifing the bits one place to the
                        // right, then store that value in VX
                        self.registers[x] = tmp_vx >> 1;
                    },

                    0x7 => {
                        // 8XY7: Set VX to result of VY - VX
                        self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
                    },

                    0xE => {
                        // 8XYE: Similar to 8XY6, except we get the most significant bit and
                        // multiply VX by 2 instead.

                        // If the most-significant bit of VX is 1, then VF is set to 1,
                        // otherwise to 0. Then VX is multiplied by 2.

                        if self.og_behaviour {
                            self.registers[x] = self.registers[y];
                        }

                        let tmp_vx = self.registers[x];

                        // To get the most significant bit (MSB), we shift the byte 7 bits to the
                        // right to isolate the highest value bit.
                        let msb: u8 = tmp_vx >> 7;

                        // Directly set VF to 1 or 0 depending on MSB of VX
                        self.registers[15] = msb;

                        // Shifting the bits left by 1 is essentially the same as multiplying the
                        // value by 2
                        self.registers[x] = tmp_vx << 1;
                    },
                    _ => unknown_opcode(opcode),
                };
            },

            0x9 => {
                // 9XY0: Skip next instruction if VX and VY are NOT equal.
                if self.registers[x] != self.registers[y] {
                    self.prog_ctr += 2;
                }
            },

            0xA => {
                // ANNN: Set value of index register to nnn.
                self.index_reg = nnn;
            },

            0xB => {
                // BNNN (OG): Jump to NNN + V0
                // BXNN (Modern): Jump to XNN (NNN) + VX
                if self.og_behaviour {
                    self.prog_ctr = nnn + self.registers[0] as u16;
                } else {
                    self.prog_ctr = nnn as u16 + self.registers[x] as u16;
                }
            },

            0xC => {
                // CXNN: Set VX to random byte AND nn.
                // First, generate a random number between 0 - 255, then perform a bitwise
                // AND on it with the value in nn, finally storing the result in VX.
                let random_byte: u8 = rand::random();
                self.registers[x] = random_byte & nn;
            },

            // Display
            0xD => {
                // DXYN: Draw to screen and check collisions.
                // Apply modulo to wrap sprites around edges if needed.
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
                let key: usize = self.registers[x] as usize;

                match nn {
                    0x9E => {
                        // EX9A: Skip next instruction if the key with value in VX is pressed
                        if self.keypad[key] {
                            self.prog_ctr += 2;
                        }
                    },

                    0xA1 => {
                        // EXA1: Skip the next instruction if key with value in VX is not pressed
                        if !self.keypad[key] {
                            self.prog_ctr += 2;
                        }
                    },

                    _ => println!("Unknown 0xE group instruction"),
                };
            },

            // Timers, memory and fonts
            0xF => {
                match nn {
                    0x07 => {
                        // FX07: Set VX to value of delay timer
                        self.registers[x] = self.delay_timer;
                    },

                    0x15 => {
                        // FX15: Set delay timer equal to VX
                        self.delay_timer = self.registers[x];
                    },

                    0x1E => {
                        // FX1E: Add the value of index_reg and VX, storing the result in index_reg
                        self.index_reg = self.index_reg + self.registers[x] as u16;
                    },

                    0x33 => {
                        // FX33: Take the hundreds, tens and ones digits from VX and place them
                        // in memory locations index_reg, index_reg + 1 and index_reg + 2 respectively
                        let tmp_vx = self.registers[x];

                        // The method for digit is as follows:
                        // Ones - Dividing by 10 leaves the remainder equal to its original last digit
                        // Tens - Divide by 10 first to remove a decimal place, then get the
                        // remainder of another division by 10
                        // Hundreds - Same as tens, but divide by 100 first to remove two decimal
                        // spots before getting the 10 remainder

                        self.memory[self.index_reg as usize + 2] = tmp_vx % 10;       // Ones
                        self.memory[self.index_reg as usize + 1] = tmp_vx / 10 % 10;  // Tens
                        self.memory[self.index_reg as usize]     = tmp_vx / 100 % 10; // Hundreds
                    },

                    0x55 => {
                        // FX55: Store registers V0 - VX in memory, starting at index_reg
                        let start = self.index_reg as usize;

                        // Copy value from registers V0 - VX, starting at the memory
                        // address of index_reg
                        self.memory[start..(start + x + 1)]
                            .copy_from_slice(&self.registers[..(x + 1)]);

                        // Early hardware had a quirk where index_reg would be
                        // incremented by (x + 1) at the end of the copy operation
                        if self.og_behaviour {
                            self.index_reg = self.index_reg.wrapping_add((x as u16) + 1);
                        }
                    },

                    0x65 => {
                        // FX65: Read registers V0 - VX from memory, starting at index_reg

                        // Copy x + 1 bytes from RAM into V registers, from V0 - VX
                        // TODO: Rewrite in a more idiomatic way
                        let mut i = 0;
                        while i <= x {
                            self.registers[i] = self.memory[self.index_reg as usize + i];
                            i += 1;
                        }

                        if self.og_behaviour {
                            self.index_reg = self.index_reg.wrapping_add((x as u16) + 1);
                        }
                    },

                    _ => unknown_opcode(opcode),
                };
            },

            _ => unknown_opcode(opcode),
        };
        // -----------------------------------------------------------------
    }
}

fn unknown_opcode(oc: u16) -> ! {
    panic!("Unimplemented: {:#06X}", oc);
}
