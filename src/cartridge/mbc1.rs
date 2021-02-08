use alloc::vec::{self, Vec};
use bitfield::BitRange;

use crate::ROM;

enum BankMode {
    ROM,
    RAM,
}

pub struct MBC1 {
    rom: ROM,
    rom_bank: u8,
    ram: Vec<u8>,
    ram_bank: u8,
    ram_enabled: bool,
    bank_mode: BankMode,
}

impl MBC1 {
    pub fn new(rom: ROM, ram_size: usize) -> Self {
        MBC1 {
            rom,
            rom_bank: 1,
            ram: vec::from_elem(0, ram_size),
            ram_bank: 0,
            ram_enabled: false,
            bank_mode: BankMode::ROM,
        }
    }

    pub fn rom(&self) -> &ROM {
        &self.rom
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // ROM Bank 00 (Read Only)
            0x0000..=0x3FFF => self.rom.read(addr), //self.rom[usize::from(addr)],
            // ROM Bank 01-7F (Read Only)
            0x4000..=0x7FFF => {
                let rom_bank = match self.rom_bank {
                    0x00 => 0x01,
                    0x20 => 0x21,
                    0x40 => 0x41,
                    0x60 => 0x61,
                    bank => bank,
                };

                let offset = u16::from(rom_bank) * 0x4000;
                self.rom.read(addr - 0x4000 + offset)
                //self.rom[usize::from(addr) - 0x4000 + offset]
            }
            // RAM Bank 00-03, if any (Read/Write)
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return 0xFF;
                }

                let offset = usize::from(self.ram_bank) * 0x2000;
                self.ram[usize::from(addr) - 0xA000 + offset]
            }
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM Enable (Write Only)
            0x0000..=0x1FFF => {
                self.ram_enabled = BitRange::<u8>::bit_range(&value, 3, 0) == 0x0A;
            }
            // ROM Bank Number (Write Only)
            0x2000..=0x3FFF => {
                self.rom_bank.set_bit_range(4, 0, value);
            }
            // RAM Bank Number - or - Upper Bits of ROM Bank Number (Write Only)
            0x4000..=0x5FFF => match self.bank_mode {
                BankMode::ROM => {
                    self.rom_bank.set_bit_range(6, 5, value);
                }
                BankMode::RAM => match value {
                    0x00..=0x03 => self.ram_bank = value,
                    _ => unreachable!(),
                },
            },
            // ROM/RAM Mode Select (Write Only)
            0x6000..=0x7FFF => {
                self.bank_mode = match value {
                    0x00 => BankMode::ROM,
                    0x01 => BankMode::RAM,
                    _ => unreachable!(),
                }
            }
            // RAM Bank 00-03, if any (Read/Write)
            0xA000..=0xBFFF => {
                if self.ram.len() > 0 && self.ram_enabled {
                    let offset = usize::from(self.ram_bank) * 0x2000;
                    self.ram[usize::from(addr) - 0xA000 + offset] = value;
                }
            }
            _ => unreachable!(),
        }
    }
}
