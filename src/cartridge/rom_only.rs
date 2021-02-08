use crate::ROM;

pub struct ROMOnly(ROM);

impl ROMOnly {
    pub fn new(rom: ROM) -> Self {
        ROMOnly(rom.into())
    }

    pub fn rom(&self) -> &ROM {
        &self.0
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.0.read(addr),
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, _: u16, _: u8) {}
}
