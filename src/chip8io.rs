use std::time::Duration;

use crossterm::{
    event::{poll, read, Event},
    terminal,
};

use crate::Chip8;

impl Chip8 {
    pub fn read_key_async(&self) -> Option<usize> {
        terminal::enable_raw_mode().unwrap();
        if poll(Duration::from_millis(1)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => match event.modifiers {
                    crossterm::event::KeyModifiers::CONTROL
                        if event.code == crossterm::event::KeyCode::Char('c') =>
                    {
                        std::process::exit(0);
                    }
                    _ => match event.code {
                        crossterm::event::KeyCode::Char(c) if INPUT_KEYS.contains(&c) => {
                            return INPUT_KEYS.iter().position(|&e| e == c);
                        }
                        _ => (),
                    },
                },
                _ => (),
            };
        }
        terminal::disable_raw_mode().unwrap();
        None
    }

    pub fn read_key_sync(&self) -> Option<usize> {
        terminal::enable_raw_mode().unwrap();
        loop {
            match read().unwrap() {
                Event::Key(event) => match event.modifiers {
                    crossterm::event::KeyModifiers::CONTROL
                        if event.code == crossterm::event::KeyCode::Char('c') =>
                    {
                        std::process::exit(0);
                    }
                    _ => match event.code {
                        crossterm::event::KeyCode::Char(c) if INPUT_KEYS.contains(&c) => {
                            terminal::disable_raw_mode().unwrap();
                            return INPUT_KEYS.iter().position(|&e| e == c);
                        }
                        _ => (),
                    },
                },
                _ => (),
            };
        }
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
}

static INPUT_KEYS: [char; 16] = [
    '\x31', '\x32', '\x33', '\x34', '\x71', '\x77', '\x65', '\x72', '\x61', '\x73', '\x64', '\x66',
    '\x7A', '\x78', '\x63', '\x76',
];
