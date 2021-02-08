use gb::Color;
use gb::Gameboy;
use gb::Joypad;
use gb::HAL;
use gb::ROM;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

struct TestHAL {
    output: Vec<char>,
}

impl HAL for TestHAL {
    fn is_joypad_pressed(&self, _: Joypad) -> bool {
        false
    }

    fn put_pixel(&mut self, _: usize, _: usize, _: Color) {}

    fn serial_callback(&mut self, value: u8) -> u8 {
        print!("{}", value as char);

        use std::io::Write;
        std::io::stdout().flush().unwrap();

        self.output.push(value as char);

        0xFF
    }
}

fn run_test<P: AsRef<Path>>(path: P) -> String {
    let rom = ROM::from(std::fs::read(path).unwrap());
    let hal = Rc::new(RefCell::new(TestHAL { output: Vec::new() }));

    {
        let mut gameboy = Gameboy::new(rom, hal.clone());

        for _ in 0..30000000 {
            gameboy.step();
        }
    }

    let output = hal.borrow_mut().output.iter().collect::<String>();
    output
}

#[test]
fn cpu_instrs() {
    let output = run_test("test_roms/cpu_instrs/cpu_instrs.gb");

    assert!(output.contains("Passed all tests"))
}

#[test]
fn instr_timing() {
    let output = run_test("test_roms/instr_timing/instr_timing.gb");

    assert!(output.contains("Passed"))
}

#[test]
fn mem_timing() {
    let output = run_test("test_roms/mem_timing/mem_timing.gb");

    assert!(output.contains("Passed all tests"))
}
