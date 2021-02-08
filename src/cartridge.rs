mod mbc1;
mod rom_only;

use mbc1::MBC1;
use rom_only::ROMOnly;

use crate::ROM;

pub enum Cartridge {
    ROMOnly(ROMOnly),
    MBC1(MBC1),
}

impl Cartridge {
    pub fn rom_only(rom: ROM) -> Self {
        Cartridge::ROMOnly(ROMOnly::new(rom))
    }

    pub fn mbc1(rom: ROM, ram_size: usize) -> Self {
        Cartridge::MBC1(MBC1::new(rom, ram_size))
    }
}

impl Cartridge {
    pub fn rom(&self) -> &ROM {
        match self {
            Cartridge::ROMOnly(rom) => rom.rom(),
            Cartridge::MBC1(mbc1) => mbc1.rom(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match self {
            Cartridge::ROMOnly(rom) => rom.read(addr),
            Cartridge::MBC1(mbc1) => mbc1.read(addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match self {
            Cartridge::ROMOnly(rom) => rom.write(addr, value),
            Cartridge::MBC1(mbc1) => mbc1.write(addr, value),
        }
    }
}
