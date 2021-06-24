use chip8::{self, draw_graphics, setup_graphics, setup_intput, Chip8};
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
