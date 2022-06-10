use chip8::Chip8;
use std::fs::File;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please specify the path of the game you wish to play");
        std::process::exit(1);
    }
    let mut game = File::open(&args[1]).unwrap();
    let mut my_chip8 = Chip8::new();
    my_chip8.load_fontset();
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
            my_chip8.draw_graphics();
        }
    }
}
