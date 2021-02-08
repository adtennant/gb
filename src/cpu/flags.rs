#![allow(non_upper_case_globals)]

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct Flags : u8 {
        const Zero      = 0b1000_0000;
        const Subtract  = 0b0100_0000;
        const HalfCarry = 0b0010_0000;
        const Carry     = 0b0001_0000;
    }
}
