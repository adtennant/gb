use clap::{App, Arg};
use gb::{Color, Gameboy, Joypad, ROM};
use std::{cell::RefCell, rc::Rc};

struct HAL;

impl gb::HAL for HAL {
    fn is_joypad_pressed(&self, _: Joypad) -> bool {
        false
    }

    fn put_pixel(&mut self, _: usize, _: usize, _: Color) {}

    fn serial_callback(&mut self, value: u8) -> u8 {
        print!("{}", value as char);

        use std::io::Write;
        std::io::stdout().flush().unwrap();

        0xFF
    }
}

fn main() {
    let matches = App::new("Gameboy")
        .version("1.0")
        .author("Alex Tennant <alex@adtennant.co.uk>")
        .about("Plays Gameboy ROMs")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the ROM file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let bytes = std::fs::read(matches.value_of("INPUT").unwrap()).unwrap();
    let rom = ROM::from(bytes);
    let hal = Rc::new(RefCell::new(HAL));

    let mut gameboy = Gameboy::new(rom, hal);

    loop {
        gameboy.step();
    }
}
