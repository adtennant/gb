use bitfield::bitfield;
use std::fmt;

#[derive(Debug)]
pub enum Frequency {
    _4096,
    _262144,
    _65536,
    _16384,
}

impl Frequency {
    fn into_mask(self) -> u16 {
        match self {
            Frequency::_4096 => 0b0000_0010_0000_0000,
            Frequency::_16384 => 0b0000_0000_1000_0000,
            Frequency::_65536 => 0b0000_0000_0010_0000,
            Frequency::_262144 => 0b0000_0000_0000_1000,
        }
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Frequency::_4096 => write!(f, "4096 Hz"),
            Frequency::_262144 => write!(f, "262144 Hz"),
            Frequency::_65536 => write!(f, "65536 Hz"),
            Frequency::_16384 => write!(f, "16384 Hz"),
        }
    }
}

impl From<u8> for Frequency {
    fn from(value: u8) -> Self {
        match value {
            0b00 => Frequency::_4096,
            0b01 => Frequency::_262144,
            0b10 => Frequency::_65536,
            0b11 => Frequency::_16384,
            _ => unreachable!(),
        }
    }
}

bitfield! {
    #[derive(Clone)]
    struct Counter(u16);
    //impl Debug;
    u8;
    div, _: 15, 8;
}

bitfield! {
    #[derive(Clone)]
    pub struct TAC(u8);
    //impl Debug;
    u8;
    pub enabled, _: 2;
    pub into Frequency, frequency, _: 1, 0;
}

impl Into<u8> for TAC {
    fn into(self) -> u8 {
        self.0
    }
}

#[derive(Clone)]
pub struct Timer {
    counter: Counter,
    tac: TAC,
    tima: u8,
    tma: u8,

    overflow: bool,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            counter: Counter(0xABCC),
            tac: TAC(0x00),
            tima: 0x00,
            tma: 0x00,

            overflow: false,
        }
    }

    pub fn div(&self) -> u8 {
        self.counter.div()
    }

    pub fn tac(&self) -> TAC {
        self.tac.clone()
    }

    pub fn tima(&self) -> u8 {
        self.tima
    }

    pub fn tma(&self) -> u8 {
        self.tma
    }

    pub fn div_write_m_cycle(&mut self, _: u8) -> bool {
        let previous = self.clone();
        let should_interrupt = self.tick_m_cycle();

        // Value resets to 0 no matter what the value written is
        self.counter = Counter(0);

        self.try_increment_tima(&previous);

        should_interrupt
    }

    pub fn tac_write_m_cycle(&mut self, value: u8) -> bool {
        let previous = self.clone();
        let should_interrupt = self.tick_m_cycle();

        // Only the lower 3 bits are (R/W).
        self.tac = TAC(value | 0b1111_1000);

        self.try_increment_tima(&previous);

        should_interrupt
    }

    pub fn tima_write_m_cycle(&mut self, value: u8) -> bool {
        let previous_overflow = self.overflow;
        let should_interrupt = self.tick_m_cycle();

        if !previous_overflow {
            // If you write to TIMA during the cycle that TMA is loaded
            // to it, the write will be ignored
            self.tima = value;
        };

        if self.overflow && !previous_overflow {
            // You can prevent IF being set and prevent the TIMA from
            // being reloaded from TMA by writing a value to TIMA.
            self.overflow = false;
        }

        should_interrupt
    }

    pub fn tma_write_m_cycle(&mut self, value: u8) -> bool {
        let previous_overflow = self.overflow;
        let should_interrupt = self.tick_m_cycle();

        if previous_overflow {
            // If TMA is written the same cycle it is loaded to TIMA,
            // TIMA is also loaded with that value.
            self.tima = value;
        };

        self.tma = value;

        should_interrupt
    }

    fn should_increment_tima(&self) -> bool {
        self.tac.enabled() && (self.counter.0 & self.tac.frequency().into_mask()) > 0
    }

    fn try_increment_tima(&mut self, previous: &Timer) {
        let old_bit = previous.should_increment_tima();
        let new_bit = self.should_increment_tima();

        if old_bit && !new_bit {
            // Increment TIMA on the falling edge
            let (tima, overflow) = self.tima.overflowing_add(1);

            self.tima = tima;
            self.overflow = overflow;
        }
    }

    pub fn tick_m_cycle(&mut self) -> bool {
        let previous = self.clone();
        let should_interrupt = self.overflow;

        if self.overflow {
            self.tima = self.tma;
            self.overflow = false;
        }

        self.counter = Counter(self.counter.0.wrapping_add(4));

        self.try_increment_tima(&previous);

        should_interrupt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_initialised_correctly() {
        let timer = Timer::new();

        assert_eq!(0xAB, timer.div());
        assert_eq!(0x00, timer.tac().0);
        assert_eq!(0x00, timer.tima());
        assert_eq!(0x00, timer.tma());
    }
}
