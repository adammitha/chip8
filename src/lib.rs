#[allow(dead_code)]
mod cycle;
use std::{u16, u8};

/// A Chip8 emulator
pub struct Chip8 {
    opcode: u16,
    memory: [u8; 4096],
    stack: [u16; 16],
    sp: u16,
    /// General-purpose registers
    v: [u8; 16],
    /// Index register
    i: u16,
    pc: u16,
    /// Graphics
    gfx: [u8; 64 * 32],
    pub draw_flag: bool,
    key: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    /// Constructs and initializes a new Chip8 emulator
    pub fn new() -> Self {
        Self {
            opcode: 0,
            memory: [0 as u8; 4096],
            stack: [0 as u16; 16],
            sp: 0 as u16,
            v: [0 as u8; 16],
            i: 0 as u16,
            pc: 0x200 as u16,
            gfx: [0 as u8; 64 * 32],
            draw_flag: false,
            key: [0 as u8; 16],
            delay_timer: 0 as u8,
            sound_timer: 0 as u8,
        }
    }

    pub fn load_game(&mut self, game: &str) {
        todo!()
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        self.opcode = self.fetch();
        // Decode and execute
        self.exec();
    }

    pub fn set_keys(&self) {
        todo!()
    }
}

pub fn draw_graphics() -> () {
    todo!()
}

pub fn setup_graphics() -> () {
    todo!()
}

pub fn setup_intput() -> () {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_fetch() {
        let mut test_chip8 = Chip8::new();
        test_chip8.memory[0] = 0xA2;
        test_chip8.memory[1] = 0xF0;
        test_chip8.pc = 0;
        assert_eq!(test_chip8.fetch(), 0xA2F0 as u16);
    }
}
