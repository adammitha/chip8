mod chip8io;
mod cycle;
mod duration;
use std::{
    io,
    time::{Duration, Instant},
    u16, u8,
};

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
    delay_timer: u8,
    sound_timer: u8,
    timer_reset_instance: Instant,
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
            delay_timer: 0 as u8,
            sound_timer: 0 as u8,
            timer_reset_instance: Instant::now(),
        }
    }

    pub fn load_fontset(&mut self) {
        for (index, &val) in CHIP8_FONTSET.iter().enumerate() {
            self.memory[index] = val;
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
        self.update_timers();
    }

    fn update_timers(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.timer_reset_instance) > Duration::from_millis(17) {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                if self.sound_timer == 1 {
                    println!("BEEP!");
                }
                self.sound_timer -= 1;
            }
            self.timer_reset_instance = Instant::now();
        }
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
