use alloc::vec::Vec;
use std::fmt;

use crate::cartridge::Cartridge;

pub enum CartridgeType {
    ROMOnly,
    MBC1,
}

impl fmt::Display for CartridgeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CartridgeType::ROMOnly => write!(f, "ROM ONLY"),
            CartridgeType::MBC1 => write!(f, "MBC1"),
        }
    }
}

pub struct ROM(Vec<u8>);

impl From<Vec<u8>> for ROM {
    fn from(value: Vec<u8>) -> Self {
        ROM(value)
    }
}

impl ROM {
    /*pub fn read<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        std::fs::read(path).map_err(|e| e.to_string()).map(ROM)
    }*/
    pub fn title(&self) -> String {
        self.0[0x134..0x144]
            .iter()
            .take_while(|&&c| c != 0)
            .map(|&c| c as char)
            .collect()
    }

    pub fn manufacturer_code(&self) -> String {
        self.0[0x13F..0x144].iter().map(|&c| c as char).collect()
    }

    pub fn cgb_flag(&self) -> u8 {
        self.0[0x143]
    }

    pub fn new_licensee_code(&self) -> String {
        self.0[0x144..0x146].iter().map(|&c| c as char).collect()
    }

    pub fn sgb_flag(&self) -> u8 {
        self.0[0x146]
    }

    pub fn cartridge_type(&self) -> CartridgeType {
        match self.0[0x147] {
            0x00 => CartridgeType::ROMOnly,
            0x01..=0x03 => CartridgeType::MBC1,
            value => unimplemented!("{:#04X}", value),
        }
    }

    pub fn rom_size(&self) -> usize {
        32768 << self.0[0x148]
    }

    pub fn ram_size(&self) -> usize {
        match self.0[0x149] {
            0x00 => 0,
            0x01 => 2048,
            0x02 => 8192,
            0x03 => 32768,
            _ => unreachable!(),
        }
    }

    pub fn destination_code(&self) -> u8 {
        self.0[0x14A]
    }

    pub fn old_licensee_code(&self) -> u8 {
        self.0[0x14B]
    }

    pub fn mask_rom_version(&self) -> u8 {
        self.0[0x14C]
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.0[usize::from(addr)]
    }
}

impl Into<Cartridge> for ROM {
    fn into(self) -> Cartridge {
        match self.cartridge_type() {
            CartridgeType::ROMOnly => Cartridge::rom_only(self),
            CartridgeType::MBC1 => {
                let ram_size = self.ram_size();
                Cartridge::mbc1(self, ram_size)
            }
        }
    }
}
