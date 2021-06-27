use chip8::Chip8;
use crossterm::{cursor, QueueableCommand};
use std::fs::File;
use std::io::{stdout, Write};
use std::time::Instant;

fn main() {
    let mut my_chip8 = Chip8::new();
    my_chip8.load_fontset();
    let mut game = File::open(
        "/Users/adammitha/Documents/Personal/Development/rust/chip8/games/octojam1title.ch8",
    )
    .unwrap();
    my_chip8.load_game(&mut game).unwrap();

    loop {
        let t1 = Instant::now();
        my_chip8.emulate_cycle();
        let t2 = Instant::now();
        let diff = t2.duration_since(t1);
        if diff < my_chip8.duration() {
            std::thread::sleep(my_chip8.duration() - diff);
        }

        if my_chip8.draw_flag {
            let out = my_chip8.draw_graphics();
            let mut stdout = stdout();
            stdout.queue(cursor::SavePosition).unwrap();
            stdout.write(out.as_bytes()).unwrap();
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.flush().unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}
