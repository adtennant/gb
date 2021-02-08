use alloc::rc::Rc;
use bitfield::bitfield;
use core::cell::RefCell;
use std::fmt;

use super::hal::HAL;

pub enum ShiftClock {
    External,
    Internal,
}

impl From<u8> for ShiftClock {
    fn from(value: u8) -> Self {
        match value {
            0 => ShiftClock::External,
            1 => ShiftClock::Internal,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for ShiftClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ShiftClock::External => write!(f, "External"),
            ShiftClock::Internal => write!(f, "Internal"),
        }
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct SC(u8);
    //impl Debug;
    u8;
    pub transfer_start, set_transfer_start: 7;
    pub into ShiftClock, shift_clock, _: 0, 0;
}

impl Into<u8> for SC {
    fn into(self) -> u8 {
        self.0
    }
}

pub struct Serial {
    sb: u8,
    sc: SC,

    hal: Rc<RefCell<dyn HAL>>,
}

impl Serial {
    pub fn new(hal: Rc<RefCell<dyn HAL>>) -> Self {
        Serial {
            sb: 0x00,
            sc: SC(0x00),

            hal,
        }
    }

    pub fn sb(&self) -> u8 {
        self.sb
    }

    pub fn sc(&self) -> SC {
        self.sc.clone()
    }

    pub fn set_sb(&mut self, value: u8) {
        self.sb = value;
    }

    pub fn set_sc(&mut self, value: u8) {
        self.sc = SC(value);
    }

    pub fn tick_m_cycle(&mut self) -> bool {
        if self.sc.transfer_start() {
            self.sb = self.hal.borrow_mut().serial_callback(self.sb);
            self.sc.set_transfer_start(false);
        }

        false

        /*let serial = &mut gb.serial;

        if serial.sc.transfer_start() {
            serial.transfer_cycles = serial.transfer_cycles.wrapping_add(4);
        }

        // TODO: Figure this out, 8192Hz
        if serial.transfer_cycles > 0 {
            //(serial.send_func)(serial.sb);

            print!("{}", serial.sb as char);
            use std::io::Write;
            std::io::stdout().flush().unwrap();

            serial.sb = 0xFF; //(serial.receive_func)();
            serial.sc.set_transfer_start(false);
            serial.transfer_cycles = 0;
        }*/
    }
}
