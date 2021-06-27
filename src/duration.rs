use std::time::Duration;

use crate::Chip8;

impl Chip8 {
    /// Returns expected execution duration of the current opcode
    pub fn duration(&self) -> Duration {
        let ins_op0 = (0xf000 & self.opcode) >> 12;
        let ins_op1 = (0x0f00 & self.opcode) >> 8;
        let ins_op2 = (0x00f0 & self.opcode) >> 4;
        let ins_op3 = 0x000f & self.opcode;
        let t = match ins_op0 {
            0x0 => match ins_op1 {
                0x0 => match ins_op2 {
                    0xE => match ins_op3 {
                        0x0 => 109,
                        0xE => 105,
                        _ => panic!("Invalid opcode: {}", self.opcode),
                    },
                    _ => panic!("Invalid opcode: {}", self.opcode),
                },
                _ => panic!("Invalid opcode: {}", self.opcode),
            },
            0x1 => 105,
            0x2 => 105,
            0x3 => 55,
            0x4 => 55,
            0x5 => 73,
            0x6 => 27,
            0x7 => 45,
            0x8 => 200,
            0x9 => 73,
            0xA => 55,
            0xB => 105,
            0xC => 164,
            0xD => 22734,
            0xE => 73,
            0xF => match ins_op2 << 4 | ins_op3 {
                0x07 => 45,
                0x0A => 0,
                0x15 => 45,
                0x18 => 45,
                0x1E => 86,
                0x29 => 91,
                0x33 => 927,
                0x55 => 605,
                0x65 => 605,
                _ => panic!("Invalid opcode: {}", self.opcode),
            },
            _ => panic!("Invalid opcode: {}", self.opcode),
        };
        Duration::from_micros(t)
    }
}
