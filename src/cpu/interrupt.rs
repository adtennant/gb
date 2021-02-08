#[derive(Copy, Clone)]
pub enum Interrupt {
    VBlank = 0,
    LCDStat = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

impl Interrupt {
    pub fn to_vector(self) -> u16 {
        match self {
            Interrupt::VBlank => 0x0040,
            Interrupt::LCDStat => 0x0048,
            Interrupt::Timer => 0x0050,
            Interrupt::Serial => 0x0058,
            Interrupt::Joypad => 0x0060,
        }
    }
}
