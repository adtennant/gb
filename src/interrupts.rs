use crate::Interrupt;
use bitfield::bitfield;

bitfield! {
    #[derive(Clone)]
    pub struct INTF(u8);
    //impl Debug;
    u8;
    pub joypad, set_joypad: 4;
    pub serial, set_serial: 3;
    pub timer, set_timer: 2;
    pub lcd_stat, set_lcd_stat: 1;
    pub vblank, set_vblank: 0;
}

impl Into<u8> for INTF {
    fn into(self) -> u8 {
        self.0
    }
}

impl INTF {
    fn get_highest_priority(&self) -> Option<Interrupt> {
        let n = self.0.trailing_zeros() as usize;

        match n {
            0 => Some(Interrupt::VBlank),
            1 => Some(Interrupt::LCDStat),
            2 => Some(Interrupt::Timer),
            3 => Some(Interrupt::Serial),
            4 => Some(Interrupt::Joypad),
            _ => None,
        }
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct INTE(u8);
    //impl Debug;
    u8;
    pub joypad, _: 4;
    pub serial, _: 3;
    pub timer, _: 2;
    pub lcd_stat, _ : 1;
    pub vblank, _: 0;
}

impl Into<u8> for INTE {
    fn into(self) -> u8 {
        self.0
    }
}

pub struct Interrupts {
    intf: INTF,
    inte: INTE,
}

impl Interrupts {
    pub fn new() -> Self {
        Interrupts {
            intf: INTF(0x00),
            inte: INTE(0x00),
        }
    }

    pub fn intf(&self) -> INTF {
        self.intf.clone()
    }

    pub fn set_intf(&mut self, value: u8) {
        self.intf = INTF(value);
    }

    pub fn inte(&self) -> INTE {
        self.inte.clone()
    }

    pub fn set_inte(&mut self, value: u8) {
        self.inte = INTE(value);
    }

    pub fn should_handle_interrupt(&self) -> bool {
        (self.inte.0 & self.intf.0 & 0b00011111) > 0
    }

    pub fn pop_interrupt(&mut self) -> Option<Interrupt> {
        let interrupt = self.intf.get_highest_priority();

        match interrupt {
            Some(Interrupt::VBlank) => self.intf.set_vblank(false),
            Some(Interrupt::LCDStat) => self.intf.set_lcd_stat(false),
            Some(Interrupt::Timer) => self.intf.set_timer(false),
            Some(Interrupt::Serial) => self.intf.set_serial(false),
            Some(Interrupt::Joypad) => self.intf.set_joypad(false),
            _ => {}
        }

        interrupt
    }

    pub fn trigger_interrupt(&mut self, interrupt: Interrupt) {
        match interrupt {
            Interrupt::VBlank => self.intf.set_vblank(true),
            Interrupt::LCDStat => self.intf.set_lcd_stat(true),
            Interrupt::Timer => self.intf.set_timer(true),
            Interrupt::Serial => self.intf.set_serial(true),
            Interrupt::Joypad => self.intf.set_joypad(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_to_interrupt_vector() {
        assert_eq!(0x0040, Interrupt::VBlank.to_vector());
        assert_eq!(0x0048, Interrupt::LCDStat.to_vector());
        assert_eq!(0x0050, Interrupt::Timer.to_vector());
        assert_eq!(0x0058, Interrupt::Serial.to_vector());
        assert_eq!(0x0060, Interrupt::Joypad.to_vector());
    }
}
