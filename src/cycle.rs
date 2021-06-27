use rand::Rng;
use std::{u8, usize};

use crate::Chip8;

impl Chip8 {
    pub fn fetch(&mut self) -> u16 {
        let first = (self.memory[self.pc as usize] as u16) << 8;
        let second = self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        first | second
    }

    pub fn exec(&mut self) {
        let ins_op0 = (0xf000 & self.opcode) >> 12;
        let ins_op1 = (0x0f00 & self.opcode) >> 8;
        let ins_op2 = (0x00f0 & self.opcode) >> 4;
        let ins_op3 = 0x000f & self.opcode;
        match ins_op0 {
            0x0 => match ins_op1 {
                0x0 => match ins_op2 {
                    0xE => match ins_op3 {
                        0x0 => {
                            self.gfx = [0; 64 * 32];
                            self.draw_flag = true;
                        }
                        0xE => {
                            self.sp -= 1;
                            self.pc = self.stack[self.sp as usize];
                        }
                        _ => panic!("Invalid opcode: {}", self.opcode),
                    },
                    _ => panic!("Invalid opcode: {}", self.opcode),
                },
                _ => panic!("Invalid opcode: {}", self.opcode),
            },
            0x1 => {
                self.pc = ins_op1 << 8 | ins_op2 << 4 | ins_op3;
            }
            0x2 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = ins_op1 << 8 | ins_op2 << 4 | ins_op3;
            }
            0x3 => {
                if self.v[ins_op1 as usize] == (ins_op2 << 4 | ins_op3) as u8 {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.v[ins_op1 as usize] != (ins_op2 << 4 | ins_op3) as u8 {
                    self.pc += 2;
                }
            }
            0x5 => {
                if self.v[ins_op1 as usize] == self.v[ins_op2 as usize] as u8 {
                    self.pc += 2;
                }
            }
            0x6 => self.v[ins_op1 as usize] = (ins_op2 << 4 | ins_op3) as u8,
            0x7 => {
                self.v[ins_op1 as usize] =
                    (self.v[ins_op1 as usize] as u16 + (ins_op2 << 4 | ins_op3)) as u8
            }
            0x8 => match ins_op3 {
                0x0 => self.v[ins_op1 as usize] = self.v[ins_op2 as usize],
                0x1 => {
                    self.v[ins_op1 as usize] = self.v[ins_op1 as usize] | self.v[ins_op2 as usize]
                }
                0x2 => {
                    self.v[ins_op1 as usize] = self.v[ins_op1 as usize] & self.v[ins_op2 as usize]
                }
                0x3 => {
                    self.v[ins_op1 as usize] = self.v[ins_op1 as usize] ^ self.v[ins_op2 as usize]
                }
                0x4 => {
                    if self.v[ins_op2 as usize] > (0xFF - self.v[ins_op1 as usize]) {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[ins_op1 as usize] =
                        (self.v[ins_op1 as usize] as u16 + self.v[ins_op2 as usize] as u16) as u8;
                }
                0x5 => {
                    if self.v[ins_op1 as usize] > self.v[ins_op2 as usize] {
                        self.v[0xF] = 1
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[ins_op1 as usize] =
                        (self.v[ins_op1 as usize] as i16 - self.v[ins_op2 as usize] as i16) as u8;
                }
                0x6 => {
                    self.v[0xf] = self.v[ins_op1 as usize] & 0x1;
                    self.v[ins_op1 as usize] = self.v[ins_op1 as usize] >> 1;
                }
                0x7 => {
                    if self.v[ins_op2 as usize] > self.v[ins_op1 as usize] {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[ins_op1 as usize] =
                        (self.v[ins_op2 as usize] as i16 - self.v[ins_op1 as usize] as i16) as u8;
                }
                0xE => {
                    self.v[0xf] = (self.v[ins_op1 as usize] & 0x80) >> 7;
                    self.v[ins_op1 as usize] = self.v[ins_op1 as usize] << 1;
                }
                _ => panic!("Invalid opcode: {}", self.opcode),
            },
            0x9 => {
                if ins_op3 == 0 && self.v[ins_op1 as usize] != self.v[ins_op2 as usize] as u8 {
                    self.pc += 2;
                }
            }
            0xA => self.i = ins_op1 << 8 | ins_op2 << 4 | ins_op3,
            0xB => {
                self.pc = self.v[0] as u16 + (ins_op1 << 8 | ins_op2 << 4 | ins_op3);
            }
            0xC => {
                let num: u8 = rand::thread_rng().gen_range(0..255);
                self.v[ins_op1 as usize] = (ins_op2 << 4 | ins_op3) as u8 & num;
            }
            0xD => {
                let x = (self.v[ins_op1 as usize] % 64) as u16;
                let y = (self.v[ins_op2 as usize] % 32) as u16;
                let height = ins_op3;
                self.v[0xF] = 0;
                for yline in 0..height {
                    let pixel = self.memory[(self.i + yline) as usize];
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            let x_coord = (x + xline) % 64;
                            let y_coord = (y + yline) % 32;
                            let idx = ((x_coord + (y_coord * 64)) % (64 * 32)) as usize;
                            if self.gfx[idx] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[idx] ^= 1;
                        }
                    }
                }
                self.draw_flag = true;
            }
            0xE => match ins_op2 << 4 | ins_op3 {
                0x9E => match self.read_key_async() {
                    Some(i) if i as u8 == self.v[ins_op1 as usize] => self.pc += 2,
                    _ => (),
                },
                0xA1 => match self.read_key_async() {
                    Some(i) if i as u8 == self.v[ins_op1 as usize] => (),
                    _ => self.pc += 2,
                },
                _ => panic!("Invalid opcode: {}", self.opcode),
            },
            0xF => match ins_op2 << 4 | ins_op3 {
                0x07 => self.v[ins_op1 as usize] = self.delay_timer,
                0x0A => self.v[ins_op1 as usize] = self.read_key_sync().unwrap() as u8,
                0x15 => self.delay_timer = self.v[ins_op1 as usize],
                0x18 => self.sound_timer = self.v[ins_op1 as usize],
                0x1E if ins_op1 != 0xF => {
                    self.i += self.v[ins_op1 as usize] as u16;
                    if self.i > 0xFFF {
                        self.v[0xF] = 1
                    };
                }
                0x29 => self.i = ins_op1 * 8,
                0x33 => {
                    self.memory[self.i as usize] = self.v[ins_op1 as usize] / 100;
                    self.memory[(self.i + 1) as usize] = (self.v[ins_op1 as usize] / 10) % 10;
                    self.memory[(self.i + 2) as usize] = (self.v[ins_op1 as usize] % 100) % 10;
                }
                0x55 => {
                    for reg in 0 as u16..(ins_op1 + 1) {
                        self.memory[(self.i + reg) as usize] = self.v[reg as usize];
                    }
                }
                0x65 => {
                    for reg in 0 as u16..(ins_op1 + 1) {
                        self.v[reg as usize] = self.memory[(self.i + reg) as usize];
                    }
                }
                _ => panic!("Invalid opcode: {}", self.opcode),
            },
            _ => panic!("Invalid opcode: {}", self.opcode),
        }
    }
}
