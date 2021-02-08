use alloc::rc::Rc;
use core::cell::RefCell;
use std::convert::TryInto;

use super::cartridge::Cartridge;
use super::hal::HAL;
use super::interrupts::Interrupts;
use super::joypad::Joypad;
use super::ppu::PPU;
use super::serial::Serial;
use super::timer::Timer;
use super::Interrupt;

pub struct Bus {
    cartridge: Cartridge,
    ppu: PPU,
    wram: [u8; 8192],
    joypad: Joypad,
    serial: Serial,
    timer: Timer,
    interrupts: Interrupts,
    hram: [u8; 127],
}

impl Bus {
    pub fn with_cartridge(cartridge: Cartridge, hal: Rc<RefCell<dyn HAL>>) -> Self {
        Bus {
            cartridge,
            ppu: PPU::new(hal.clone()),
            wram: [0; 8192],
            joypad: Joypad::new(hal.clone()),
            serial: Serial::new(hal.clone()),
            timer: Timer::new(),
            interrupts: Interrupts::new(),
            hram: [0; 127],
        }
    }

    pub fn cartridge(&self) -> &Cartridge {
        &self.cartridge
    }

    pub fn ppu(&self) -> &PPU {
        &self.ppu
    }

    pub fn joypad(&self) -> &Joypad {
        &self.joypad
    }

    pub fn serial(&self) -> &Serial {
        &self.serial
    }

    pub fn timer(&self) -> &Timer {
        &self.timer
    }

    pub fn interrupts(&self) -> &Interrupts {
        &self.interrupts
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.read(addr),
            0x8000..=0x9FFF | 0xFE00..=0xFE9F => self.ppu.read(addr),

            0xC000..=0xFDFF => {
                // Mirrored from 0xE000..=0xFDFF
                let offset = (usize::from(addr) - 0xC000) % self.wram.len();
                self.wram[offset]
            }

            0xFF00 => self.joypad.joyp().inputs(),

            0xFF01 => self.serial.sb(),
            0xFF02 => self.serial.sc().into(),

            0xFF04 => self.timer.div(),
            0xFF05 => self.timer.tima(),
            0xFF06 => self.timer.tma(),
            0xFF07 => self.timer.tac().into(),

            0xFF40 => self.ppu.lcdc().into(),
            0xFF41 => self.ppu.stat().into(),
            0xFF42 => self.ppu.scy(),
            0xFF43 => self.ppu.scx(),
            0xFF44 => self.ppu.ly(),
            0xFF45 => self.ppu.lyc(),
            // 0xFF46 DMA
            0xFF47 => self.ppu.bgp().into(),
            0xFF48 => self.ppu.obp0().into(),
            0xFF49 => self.ppu.obp1().into(),
            0xFF4A => self.ppu.wy(),
            0xFF4B => self.ppu.wx(),

            0xFF0F => self.interrupts.intf().into(),

            0xFF80..=0xFFFE => {
                let offset = usize::from(addr) - 0xFF80;
                self.hram[offset]
            }

            0xFFFF => self.interrupts.inte().into(),

            _ => 0xFF,
        }
    }

    fn tick_m_cycle_except_timer(&mut self) {
        // TODO: oam?

        for _ in 0..4 {
            let (vblank, lcd_stat) = self.ppu.tick();

            if vblank {
                self.interrupts.trigger_interrupt(Interrupt::VBlank);
            }

            if lcd_stat {
                self.interrupts.trigger_interrupt(Interrupt::LCDStat);
            }
        }

        // TODO: apu

        if self.serial.tick_m_cycle() {
            self.interrupts.trigger_interrupt(Interrupt::Serial);
        }

        if self.joypad.tick_m_cycle() {
            self.interrupts.trigger_interrupt(Interrupt::Joypad);
        }
    }

    fn tick_m_cycle_timer(&mut self) {
        if self.timer.tick_m_cycle() {
            self.interrupts.trigger_interrupt(Interrupt::Timer);
        }
    }

    fn timer_write_m_cycle(&mut self, addr: u16, value: u8) {
        self.tick_m_cycle_except_timer();

        let should_interrupt = match addr {
            0xFF04 => self.timer.div_write_m_cycle(value),
            0xFF05 => self.timer.tima_write_m_cycle(value),
            0xFF06 => self.timer.tma_write_m_cycle(value),
            0xFF07 => self.timer.tac_write_m_cycle(value),
            _ => unreachable!(),
        };

        if should_interrupt {
            self.interrupts.trigger_interrupt(Interrupt::Timer);
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF | 0xA000..=0xBFFF => self.cartridge.write(addr, value),
            0x8000..=0x9FFF | 0xFE00..=0xFE9F => self.ppu.write(addr, value),

            0xC000..=0xFDFF => {
                // Mirrored from 0xE000..=0xFDFF
                let offset = (usize::from(addr) - 0xC000) % self.wram.len();
                self.wram[offset] = value;
            }

            0xFF00 => self.joypad.set_joyp(value),

            0xFF01 => self.serial.set_sb(value),
            0xFF02 => self.serial.set_sc(value),

            0xFF04..=0xFF07 => unreachable!(),

            0xFF0F => self.interrupts.set_intf(value),

            0xFF40 => self.ppu.set_lcdc(value),
            0xFF41 => self.ppu.set_stat(value),
            0xFF42 => self.ppu.set_scy(value),
            0xFF43 => self.ppu.set_scx(value),
            0xFF44 => self.ppu.set_ly(value),
            0xFF45 => self.ppu.set_lyc(value),
            0xFF46 => {
                let src = u16::from_le_bytes([0, value]);
                let data: [u8; 160] = (0..160)
                    .map(|offset| self.read(src + offset as u16))
                    .collect::<Vec<_>>()
                    .as_slice()
                    .try_into()
                    .unwrap();

                self.ppu.dma(data);
            }
            0xFF47 => self.ppu.set_bgp(value),
            0xFF48 => self.ppu.set_obp0(value),
            0xFF49 => self.ppu.set_obp1(value),
            0xFF4A => self.ppu.set_wy(value),
            0xFF4B => self.ppu.set_wx(value),

            0xFF80..=0xFFFE => {
                let offset = usize::from(addr) - 0xFF80;
                self.hram[offset] = value;
            }
            0xFFFF => self.interrupts.set_inte(value),

            _ => {}
        }
    }
}

impl super::cpu::Bus for Bus {
    fn pop_interrupt(&mut self) -> Option<Interrupt> {
        self.interrupts.pop_interrupt()
    }

    fn should_handle_interrupt(&self) -> bool {
        self.interrupts.should_handle_interrupt()
    }

    fn read_m_cycle(&mut self, addr: u16) -> u8 {
        self.tick_m_cycle();
        self.read(addr)
    }

    fn tick_m_cycle(&mut self) {
        self.tick_m_cycle_except_timer();
        self.tick_m_cycle_timer();
    }

    fn write_m_cycle(&mut self, addr: u16, value: u8) {
        if addr >= 0xFF04 && addr <= 0xFF07 {
            self.timer_write_m_cycle(addr, value);
            return;
        }

        self.tick_m_cycle();
        self.write(addr, value);
    }
}
