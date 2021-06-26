use chip8::Chip8;
use crossterm::{cursor, QueueableCommand};
use std::fs::File;
use std::io::{stdout, Write};

fn main() {
    let mut my_chip8 = Chip8::new();
    my_chip8.load_fontset();
    let mut game = File::open("/Users/adammitha/Downloads/br8kout.ch8").unwrap();
    match my_chip8.load_game(&mut game) {
        Ok(_) => (),
        Err(_) => todo!("Handle error opening game file"),
    };

    loop {
        my_chip8.emulate_cycle();

        if my_chip8.draw_flag {
            let out = my_chip8.draw_graphics();
            let mut stdout = stdout();
            stdout.queue(cursor::SavePosition).unwrap();
            stdout.write(out.as_bytes()).unwrap();
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.flush().unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
