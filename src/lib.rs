mod cycle;
use std::{io, u16, u8};

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

    pub fn load_fontset(&mut self) {
        for (index, &val) in CHIP8_FONTSET.iter().enumerate() {
            self.memory[index + 0x50] = val;
        }
    }

    pub fn load_game<T: io::Read>(&mut self, src: &mut T) -> io::Result<usize> {
        src.read(&mut self.memory[0x200..])
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        self.opcode = self.fetch();
        // Decode and execute
        self.exec();
        // Update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }

    pub fn set_keys(&self) {
        todo!()
    }

    pub fn draw_graphics(&mut self) -> String {
        let mut out = String::new();
        for i in 0..32 {
            for j in 0..64 {
                if self.gfx[64 * i + j] == 1 {
                    out.push_str("▆");
                } else {
                    out.push(' ');
                }
            }
            out.push('\n');
        }
        self.draw_flag = false;
        out
    }

    pub fn setup_graphics() -> () {
        todo!()
    }

    pub fn setup_input() -> () {
        todo!()
    }
}

static CHIP8_FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

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
