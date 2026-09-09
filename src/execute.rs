use crate::cpu::DecodedOpcode;

pub enum Chip8Instruction {
    _00E0,
    _00EE,
    _1NNN,
    _2NNN,
    _3XNN,
    _4XNN,
    _5XY0,
    _6XNN,
    _7XNN,
    _8XY0,
    _8XY1,
    _8XY2,
    _8XY3,
    _8XY4,
    _8XY5,
    _8XY6,
    _8XY7,
    _8XYE,
    _9XY0,
    _ANNN,
    _BNNN,
    _BXNN,
    _CXNN,
    _DXYN,
    _EX9A,
    _EXA1,
    _FX07,
    _FX0A,
    _FX15,
    _FX18,
    _FX29,
    _FX1E,
    _FX33,
    _FX55,
    _FX65,
}

pub fn get_instruction(
    decoded: &DecodedOpcode,
    og_behaviour: bool,
) -> Result<Chip8Instruction, ()> {
    match decoded.n1 {
        0x0 => match decoded.nn {
            0xE0 => return Ok(Chip8Instruction::_00E0),
            0xEE => return Ok(Chip8Instruction::_00EE),
            _ => return Err(()),
        },
        0x1 => return Ok(Chip8Instruction::_1NNN),
        0x2 => return Ok(Chip8Instruction::_2NNN),
        0x3 => return Ok(Chip8Instruction::_3XNN),
        0x4 => return Ok(Chip8Instruction::_4XNN),
        0x5 => return Ok(Chip8Instruction::_5XY0),
        0x6 => return Ok(Chip8Instruction::_6XNN),
        0x7 => return Ok(Chip8Instruction::_7XNN),
        0x8 => match decoded.n {
            // Display
            0x0 => return Ok(Chip8Instruction::_8XY0),
            0x1 => return Ok(Chip8Instruction::_8XY1),
            0x2 => return Ok(Chip8Instruction::_8XY2),
            0x3 => return Ok(Chip8Instruction::_8XY3),
            0x4 => return Ok(Chip8Instruction::_8XY4),
            0x5 => return Ok(Chip8Instruction::_8XY5),
            0x6 => return Ok(Chip8Instruction::_8XY6),
            0x7 => return Ok(Chip8Instruction::_8XY7),
            0xE => return Ok(Chip8Instruction::_8XYE),
            _ => return Err(()),
        },
        0x9 => return Ok(Chip8Instruction::_9XY0),
        0xA => return Ok(Chip8Instruction::_ANNN),
        0xB => {
            if og_behaviour {
                return Ok(Chip8Instruction::_BNNN);
            }
            return Ok(Chip8Instruction::_BXNN);
        }
        0xC => return Ok(Chip8Instruction::_CXNN),
        0xD => return Ok(Chip8Instruction::_DXYN),
        0xE => {
            // Keypad checks
            match decoded.nn {
                0x9E => return Ok(Chip8Instruction::_EX9A),
                0xA1 => return Ok(Chip8Instruction::_EXA1),
                _ => return Err(()),
            };
        }
        0xF => {
            // Timers, memory and fonts
            match decoded.nn {
                0x07 => return Ok(Chip8Instruction::_FX07),
                0x0A => return Ok(Chip8Instruction::_FX0A),
                0x15 => return Ok(Chip8Instruction::_FX15),
                0x18 => return Ok(Chip8Instruction::_FX18),
                0x29 => return Ok(Chip8Instruction::_FX29),
                0x1E => return Ok(Chip8Instruction::_FX1E),
                0x33 => return Ok(Chip8Instruction::_FX33),
                0x55 => return Ok(Chip8Instruction::_FX55),
                0x65 => return Ok(Chip8Instruction::_FX65),
                _ => return Err(()),
            };
        }
        _ => return Err(()),
    };
}
