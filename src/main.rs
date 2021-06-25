use std::fs::File;

use chip8::Chip8;
fn main() {
    Chip8::setup_graphics();
    Chip8::setup_intput();
    let mut my_chip8 = Chip8::new();
    let mut game = File::open("pong").unwrap();
    match my_chip8.load_game(&mut game) {
        Ok(_) => (),
        Err(_) => todo!("Handle error opening game file"),
    };

    loop {
        my_chip8.emulate_cycle();

        if my_chip8.draw_flag {
            Chip8::draw_graphics();
        }

        my_chip8.set_keys();
    }
}
