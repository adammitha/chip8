use core::panic;
use rand::Rng;
use std::{u8, usize};

use crate::Chip8;

impl Chip8 {
    pub fn fetch(&self) -> u16 {
        let first = (self.memory[self.pc as usize] as u16) << 8;
        let second = self.memory[(self.pc + 1) as usize] as u16;
        first | second
    }

    pub fn exec(&mut self) {
        let ins_op0 = 0xf000 & self.opcode >> 12;
        let ins_op1 = 0x0f00 & self.opcode >> 8;
        let ins_op2 = 0x00f0 & self.opcode >> 4;
        let ins_op3 = 0x000f & self.opcode;
        match ins_op0 {
            0x0 => match ins_op1 {
                0x0 => match ins_op2 {
                    0xE => match ins_op3 {
                        0x0 => todo!("Clear display"),
                        0xE => todo!("return"),
                        _ => panic!("Invalid opcode!"),
                    },
                    _ => panic!("Invalid opcode!"),
                },
                _ => todo!("Call machine code routine at address NNN"),
            },
            0x1 => self.pc = ins_op1 | ins_op2 | ins_op3,
            0x2 => todo!("Calls subroutine at NNN"),
            0x3 if self.v[ins_op1 as usize] == (ins_op2 | ins_op3) as u8 => {
                self.pc += 2;
            }
            0x4 if self.v[ins_op1 as usize] != (ins_op2 | ins_op3) as u8 => {
                self.pc += 2;
            }
            0x5 if self.v[ins_op1 as usize] == self.v[ins_op2 as usize] as u8 => {
                self.pc += 2;
            }
            0x6 => self.v[ins_op1 as usize] = (ins_op2 | ins_op3) as u8,
            0x7 => self.v[ins_op1 as usize] += (ins_op2 | ins_op3) as u8,
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
                0x4 => todo!("Vx = Vx + Vy"),
                0x5 => todo!("Vx = Vx - Vy"),
                0x6 => {
                    self.v[0xf] = self.v[ins_op1 as usize] & 0x1;
                    self.v[ins_op1 as usize] = self.v[ins_op1 as usize] >> 1;
                }
                0x7 => todo!("Vx = Vy - Vx"),
                0xE => {
                    self.v[0xf] = (self.v[ins_op1 as usize] & (0x1 << 7)) >> 7;
                    self.v[ins_op1 as usize] = self.v[ins_op1 as usize] << 1;
                }
                _ => panic!("Invalid opcode!"),
            },
            0x9 if ins_op3 == 0 && self.v[ins_op1 as usize] != self.v[ins_op2 as usize] as u8 => {
                self.pc += 2;
            }
            0xA => self.i = ins_op1 | ins_op2 | ins_op3,
            0xB => self.pc = self.v[0] as u16 + (ins_op2 | ins_op3),
            0xC => {
                let num = rand::thread_rng().gen_range(0..255);
                self.v[ins_op1 as usize] &= num;
            }
            0xD => todo!("Draw a sprite"),
            0xE => todo!("Key press"),
            0xF => match ins_op2 | ins_op3 {
                0x07 => self.v[ins_op1 as usize] = self.delay_timer,
                0x0A => todo!("key press stored in Vx"),
                0x15 => self.delay_timer = self.v[ins_op1 as usize],
                0x18 => self.sound_timer = self.v[ins_op1 as usize],
                0x1E if ins_op1 != 0xF => self.i += self.v[ins_op1 as usize] as u16,
                0x29 => todo!("Set i to location of sprite in Vx"),
                0x33 => todo!("Binary coded decimal of Vx to *i"),
                0x55 => {
                    for reg in 0 as u16..ins_op1 {
                        self.memory[(self.i + reg) as usize] = self.v[reg as usize];
                    }
                }
                0x65 => {
                    for reg in 0 as u16..ins_op1 {
                        self.v[reg as usize] = self.memory[(self.i + reg) as usize];
                    }
                }
                _ => panic!("Invalid opcode!"),
            },
            _ => panic!("Invalid opcode!"),
        }
        self.pc += 2;
    }
}