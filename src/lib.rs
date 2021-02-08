//#![no_std]
extern crate alloc;

mod bus;
mod cartridge;
mod cpu;
mod hal;
mod interrupts;
mod joypad;
mod ppu;
mod rom;
mod serial;
mod timer;
// mod ffi;
// mod rom;

pub use cpu::Flags;
pub use hal::{Color, Joypad, HAL};
pub use rom::ROM;

use alloc::rc::Rc;
use core::cell::RefCell;

use bus::Bus;
use cpu::CPU;

use cpu::Interrupt;

pub struct Gameboy {
    cpu: CPU<Bus>,
}

impl Gameboy {
    pub fn new(rom: ROM, hal: Rc<RefCell<dyn HAL>>) -> Self {
        Gameboy {
            cpu: CPU::new(Bus::with_cartridge(rom.into(), hal)),
        }
    }

    pub fn cpu(&self) -> &CPU<Bus> {
        &self.cpu
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn step_frame(&mut self) {
        let mut was_in_vblank = self.cpu.bus().ppu().in_vblank();

        // Run until we are in vblank but were not previously
        while was_in_vblank || !self.cpu.bus().ppu().in_vblank() {
            was_in_vblank = self.cpu.bus().ppu().in_vblank();
            self.cpu.step();
        }
    }
}
