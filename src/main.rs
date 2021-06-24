use std::{u16, u8};

/// A Chip8 emulator
struct Chip8 {
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
    /// Returns a new Chip8 emulator with all fields initialized to 0
    pub fn new() -> Self {
        Self {
            opcode: 0,
            memory: [0 as u8; 4096],
            stack: [0 as u16; 16],
            sp: 0 as u16,
            v: [0 as u8; 16],
            i: 0 as u16,
            pc: 0 as u16,
            gfx: [0 as u8; 64 * 32],
            draw_flag: false,
            key: [0 as u8; 16],
            delay_timer: 0 as u8,
            sound_timer: 0 as u8,
        }
    }

    pub fn load_game(&self, game: &str) {
        todo!()
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        self.opcode =
            (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize] as u16);
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

fn main() {
    setup_graphics();
    setup_intput();
    let mut my_chip8 = Chip8::new();
    my_chip8.load_game("pong");

    loop {
        my_chip8.emulate_cycle();

        if my_chip8.draw_flag {
            draw_graphics();
        }

        my_chip8.set_keys();
    }
}


