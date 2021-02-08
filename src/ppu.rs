use alloc::rc::Rc;
use bitfield::bitfield;
use bitfield::Bit;
use bitfield::BitRange;
use core::cell::RefCell;
use std::fmt;

use super::hal::Color;
use super::hal::HAL;

pub enum BackgroundTileData {
    _8800,
    _8000,
}

impl fmt::Display for BackgroundTileData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            BackgroundTileData::_8800 => write!(f, "8800 - 97FF"),
            BackgroundTileData::_8000 => write!(f, "8000 - 8FFF"),
        }
    }
}

impl From<u8> for BackgroundTileData {
    fn from(value: u8) -> Self {
        match value {
            0 => BackgroundTileData::_8800,
            1 => BackgroundTileData::_8000,
            _ => unreachable!(),
        }
    }
}

impl Into<usize> for BackgroundTileData {
    fn into(self) -> usize {
        match self {
            BackgroundTileData::_8800 => 0x8800,
            BackgroundTileData::_8000 => 0x8000,
        }
    }
}

pub enum TileMapDisplay {
    _9800,
    _9C00,
}

impl fmt::Display for TileMapDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TileMapDisplay::_9800 => write!(f, "9800 - 9BFF"),
            TileMapDisplay::_9C00 => write!(f, "9C00 - 9FFF"),
        }
    }
}

impl From<u8> for TileMapDisplay {
    fn from(value: u8) -> Self {
        match value {
            0 => TileMapDisplay::_9800,
            1 => TileMapDisplay::_9C00,
            _ => unreachable!(),
        }
    }
}

impl From<TileMapDisplay> for usize {
    fn from(value: TileMapDisplay) -> usize {
        match value {
            TileMapDisplay::_9800 => 0x9800,
            TileMapDisplay::_9C00 => 0x9C00,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Mode {
    HBlank,
    OAMRead,
    VBlank,
    VRAMRead,
}

impl From<u8> for Mode {
    fn from(value: u8) -> Self {
        match value {
            0 => Mode::HBlank,
            1 => Mode::VBlank,
            2 => Mode::OAMRead,
            3 => Mode::VRAMRead,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Mode::HBlank => write!(f, "0 (HBlank)"),
            Mode::VBlank => write!(f, "1 (VBlank)"),
            Mode::OAMRead => write!(f, "2 (OAM Read)"),
            Mode::VRAMRead => write!(f, "3 (VRAM Read)"),
        }
    }
}

pub enum OBJSize {
    _8x8,
    _8x16,
}

impl OBJSize {
    fn get_height(&self) -> i16 {
        match self {
            OBJSize::_8x8 => 8,
            OBJSize::_8x16 => 16,
        }
    }
}

impl From<u8> for OBJSize {
    fn from(value: u8) -> Self {
        match value {
            0 => OBJSize::_8x8,
            1 => OBJSize::_8x16,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for OBJSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OBJSize::_8x8 => write!(f, "8x8"),
            OBJSize::_8x16 => write!(f, "8x16"),
        }
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct LCDC(u8);
    //impl Debug;
    u8;
    pub lcd_enabled, _: 7;
    pub into TileMapDisplay, window_tile_map_display_select, _: 6, 6;
    pub window_display_enabled, _: 5;
    pub into BackgroundTileData, background_and_window_tile_data_select, _: 4, 4;
    pub into TileMapDisplay, background_tile_map_display_select, _: 3, 3;
    pub into OBJSize, obj_size, _: 2, 2;
    pub obj_display_enabled, _: 1;
    pub bg_display_enabled, _: 0;
}

impl Into<u8> for LCDC {
    fn into(self) -> u8 {
        self.0
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct STAT(u8);
    //impl Debug;
    u8;
    pub coincidence_interrupt_enabled, _: 6;
    pub oam_interrupt_enabled, _: 5;
    pub vblank_interrupt_enabled, _: 4;
    pub hblank_interrupt_enabled, _: 3;
    pub coincidence_flag, set_coincidence_flag: 2;
    pub into Mode, mode_flag, _: 1, 0;
}

impl Into<u8> for STAT {
    fn into(self) -> u8 {
        self.0
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct BGP(u8);
    //impl Debug;
    u8;
    pub into Color, color_3, _: 7, 6;
    pub into Color, color_2, _: 5, 4;
    pub into Color, color_1, _: 3, 2;
    pub into Color, color_0, _: 1, 0;
}

impl BGP {
    fn as_palette(&self) -> [Color; 4] {
        [
            self.color_0(),
            self.color_1(),
            self.color_2(),
            self.color_3(),
        ]
    }
}

impl Into<u8> for BGP {
    fn into(self) -> u8 {
        self.0
    }
}

bitfield! {
    #[derive(Clone)]
    pub struct OBP(u8);
    //impl Debug;
    u8;
    pub into Color, color_3, _: 7, 6;
    pub into Color, color_2, _: 5, 4;
    pub into Color, color_1, _: 3, 2;
}

impl OBP {
    fn as_palette(&self) -> [Color; 3] {
        [self.color_1(), self.color_2(), self.color_3()]
    }
}

impl Into<u8> for OBP {
    fn into(self) -> u8 {
        self.0
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::White,
            1 => Color::LightGrey,
            2 => Color::DarkGrey,
            3 => Color::Black,
            _ => unreachable!(),
        }
    }
}

pub struct PPU {
    vram: [u8; 8192],
    oam: [u8; 160],

    lcdc: LCDC,
    stat: STAT,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    wy: u8,
    wx: u8,
    bgp: BGP,
    obp0: OBP,
    obp1: OBP,

    counter: usize,
    mode: Mode,

    hal: Rc<RefCell<dyn HAL>>,
}

impl PPU {
    pub fn new(hal: Rc<RefCell<dyn HAL>>) -> Self {
        PPU {
            vram: [0; 8192],
            oam: [0; 160],

            lcdc: LCDC(0x91),
            stat: STAT(0x00),
            scy: 0x00,
            scx: 0x00,
            ly: 0x00,
            lyc: 0x00,
            wy: 0x00,
            wx: 0x00,
            bgp: BGP(0xFC),
            obp0: OBP(0xFF),
            obp1: OBP(0xFF),

            counter: 0,
            mode: Mode::OAMRead,

            hal,
        }
    }

    pub fn in_vblank(&self) -> bool {
        self.mode == Mode::VBlank
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => {
                if let Mode::VRAMRead = self.mode {
                    return 0xFF;
                }

                self.vram[usize::from(addr) - 0x8000]
            }
            0xFE00..=0xFE9F => {
                if let Mode::OAMRead | Mode::VRAMRead = self.mode {
                    return 0xFF;
                }

                self.oam[usize::from(addr) - 0xFE00]
            }
            _ => unreachable!(),
        }
    }

    pub fn dma(&mut self, data: [u8; 160]) {
        self.oam = data;
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => {
                if let Mode::VRAMRead = self.mode {
                    return;
                }

                self.vram[usize::from(addr) - 0x8000] = value;
            }
            0xFE00..=0xFE9F => {
                if let Mode::OAMRead | Mode::VRAMRead = self.mode {
                    return;
                }

                self.oam[usize::from(addr) - 0xFE00] = value;
            }
            _ => unreachable!(),
        }
    }

    pub fn lcdc(&self) -> LCDC {
        self.lcdc.clone()
    }

    pub fn stat(&self) -> STAT {
        self.stat.clone()
    }

    pub fn scy(&self) -> u8 {
        self.scy
    }

    pub fn scx(&self) -> u8 {
        self.scx
    }

    pub fn ly(&self) -> u8 {
        if self.lcdc.lcd_enabled() {
            self.ly
        } else {
            0
        }
    }

    pub fn lyc(&self) -> u8 {
        self.lyc
    }

    pub fn wy(&self) -> u8 {
        self.wy
    }

    pub fn wx(&self) -> u8 {
        self.wx
    }

    pub fn bgp(&self) -> BGP {
        self.bgp.clone()
    }

    pub fn obp0(&self) -> OBP {
        self.obp0.clone()
    }

    pub fn obp1(&self) -> OBP {
        self.obp1.clone()
    }

    pub fn set_lcdc(&mut self, value: u8) {
        self.lcdc = LCDC(value);
    }

    pub fn set_stat(&mut self, value: u8) {
        let mut stat = self.stat.0;
        stat.set_bit_range(6, 3, BitRange::<u8>::bit_range(&value, 6, 3));

        self.stat = STAT(stat);
    }

    pub fn set_scy(&mut self, value: u8) {
        self.scy = value;
    }

    pub fn set_scx(&mut self, value: u8) {
        self.scx = value;
    }

    pub fn set_ly(&mut self, _: u8) {
        self.ly = 0;
    }

    pub fn set_lyc(&mut self, value: u8) {
        self.lyc = value;
    }

    pub fn set_wy(&mut self, value: u8) {
        self.wy = value;
    }

    pub fn set_wx(&mut self, value: u8) {
        self.wx = value;
    }

    pub fn set_bgp(&mut self, value: u8) {
        self.bgp = BGP(value);
    }

    pub fn set_obp0(&mut self, value: u8) {
        self.obp0 = OBP(value);
    }

    pub fn set_obp1(&mut self, value: u8) {
        self.obp1 = OBP(value);
    }

    pub fn tick(&mut self) -> (bool, bool) {
        if !self.lcdc.lcd_enabled() {
            return (false, false);
        }

        self.counter = self.counter.wrapping_add(1);

        let (mut vblank, mut lcdstat) = (false, false);

        match self.mode {
            Mode::OAMRead => {
                if self.counter == 80 {
                    self.counter = 0;
                    self.mode = Mode::VRAMRead;
                }
            }
            Mode::VRAMRead => {
                // draw individual pixels instead?

                if self.counter == 172 {
                    self.counter = 0;
                    self.mode = Mode::HBlank;

                    if self.stat.hblank_interrupt_enabled() {
                        lcdstat = true;
                    }

                    self.render_scanline();
                }
            }
            Mode::HBlank => {
                if self.counter == 204 {
                    self.counter = 0;
                    self.ly = self.ly.wrapping_add(1);
                    self.stat.set_coincidence_flag(self.ly == self.lyc);

                    if self.ly == 143 {
                        self.mode = Mode::VBlank;
                        vblank = true;

                        if self.stat.vblank_interrupt_enabled() {
                            lcdstat = true;
                        }
                    } else {
                        self.mode = Mode::OAMRead;

                        if self.stat.oam_interrupt_enabled() {
                            lcdstat = true;
                        }
                    }
                }

                if self.stat.coincidence_interrupt_enabled() && self.stat.coincidence_flag() {
                    lcdstat = true;
                }
            }
            Mode::VBlank => {
                if self.counter == 456 {
                    self.counter = 0;
                    self.ly = self.ly.wrapping_add(1);
                    self.stat.set_coincidence_flag(self.ly == self.lyc);

                    if self.ly > 153 {
                        self.mode = Mode::OAMRead;

                        if self.stat.oam_interrupt_enabled() {
                            lcdstat = true;
                        }

                        self.ly = 0;
                    }
                }

                if self.stat.coincidence_interrupt_enabled() && self.stat.coincidence_flag() {
                    lcdstat = true;
                }
            }
        }

        (vblank, lcdstat)
    }

    pub fn tiles(&self) -> Vec<Vec<u8>> {
        self.vram[0..384 * 16]
            .chunks(16)
            .map(|tile_data| {
                let mut tile = Vec::new();

                for y in 0..8 {
                    for x in 0..8 {
                        let low = tile_data[y * 2].bit(7 - x);
                        let high = tile_data[y * 2 + 1].bit(7 - x);

                        let mut palette_index: u8 = 0;
                        palette_index.set_bit(0, low);
                        palette_index.set_bit(1, high);

                        tile.push(palette_index);
                    }
                }

                tile
            })
            .collect()
    }

    pub fn background_tile_map(&self) -> Vec<Color> {
        let background_palette = self.bgp.as_palette();
        let background_tile_map_vram_offset =
            usize::from(self.lcdc.background_tile_map_display_select()) - 0x8000;

        let tiles = self.tiles();

        let mut background_tile_map = [Color::White; 32 * 8 * 32 * 8];

        for tile_map_row in 0..32 {
            for tile_map_column in 0..32 {
                let tile_map_offset = tile_map_row * 32 + tile_map_column;
                let tile_index =
                    self.vram[background_tile_map_vram_offset + tile_map_offset] as usize;

                if let BackgroundTileData::_8800 =
                    self.lcdc.background_and_window_tile_data_select()
                {
                    return [Color::White; 32 * 8 * 32 * 8].to_vec(); //unimplemented!();
                }

                //let tile_vram_offset = tile_index * 16;
                //let tile_data = &self.vram[tile_vram_offset..tile_vram_offset + 16];
                let tile = &tiles[tile_index];

                for x in 0..8 {
                    for y in 0..8 {
                        /*let low = tile_data[y * 2].bit(7 - x);
                        let high = tile_data[y * 2 + 1].bit(7 - x);

                        let mut palette_index: u8 = 0;
                        palette_index.set_bit(0, low);
                        palette_index.set_bit(1, high);*/
                        let palette_index = tile[y * 8 + x];

                        let background_tile_map_x = tile_map_column * 8 + x;
                        let background_tile_map_y = tile_map_row * 8 + y;
                        let background_tile_map_index =
                            background_tile_map_y * 32 * 8 + background_tile_map_x;

                        background_tile_map[background_tile_map_index] =
                            background_palette[palette_index as usize];
                    }
                }
            }
        }

        background_tile_map.to_vec()
    }

    fn render_scanline(&mut self) {
        let line = usize::from(self.ly);

        // Background
        let background_palette = self.bgp.as_palette();
        let background_tile_map_vram_offset =
            usize::from(self.lcdc.background_tile_map_display_select()) - 0x8000;

        // Sprites
        let obj_size = self.lcdc.obj_size();

        // TODO: Limit number per line
        let sprites_on_line: Vec<_> = self
            .oam
            .chunks(4)
            .filter(|s| s[0] > 0 && s[0] < 160)
            .filter(|s| {
                let sprite_min_y = s[0] as i16 - 16;
                let sprite_max_y = sprite_min_y + obj_size.get_height();

                line as i16 >= sprite_min_y && (line as i16) < sprite_max_y
            })
            .collect();

        for x in 0..160 {
            let color = if self.lcdc.bg_display_enabled() {
                let tile_map_x = x / 8;
                let tile_map_y = line / 8;

                let tile_map_offset = tile_map_y * 32 + tile_map_x;

                let tile_index =
                    self.vram[background_tile_map_vram_offset + tile_map_offset] as usize;

                if let BackgroundTileData::_8800 =
                    self.lcdc.background_and_window_tile_data_select()
                {
                    unimplemented!();
                }

                let tile_vram_offset = tile_index * 16;
                let tile_data = &self.vram[tile_vram_offset..tile_vram_offset + 16];

                let tile_x = x % 8;
                let tile_y = line % 8;

                let low = tile_data[tile_y * 2].bit(7 - tile_x);
                let high = tile_data[tile_y * 2 + 1].bit(7 - tile_x);

                let mut palette_index: u8 = 0;
                palette_index.set_bit(0, low);
                palette_index.set_bit(1, high);

                background_palette[palette_index as usize]
            } else {
                Color::White
            };

            let color = if self.lcdc.obj_display_enabled() {
                // TODO: Priorities
                let sprite = sprites_on_line
                    .iter()
                    .filter(|sprite| {
                        let sprite_max_x = sprite[1] as i16;
                        let sprite_min_x = sprite_max_x - 8;

                        x as i16 >= sprite_min_x && (x as i16) < sprite_max_x
                    })
                    .next();

                if let Some(sprite) = sprite {
                    let sprite_y = sprite[0] as i16 - 16;
                    let sprite_x = sprite[1] as i16 - 8;

                    let tile_y = (line as i16 - sprite_y) as usize;
                    let tile_x = (x as i16 - sprite_x) as usize;

                    // TODO: 8x16
                    let tile_index = sprite[2] as usize;

                    let palette = if !sprite[3].bit(4) {
                        self.obp0.as_palette()
                    } else {
                        self.obp1.as_palette()
                    };

                    let tile_vram_offset = tile_index * 16;
                    let tile_data = &self.vram[tile_vram_offset..tile_vram_offset + 16];

                    let low = tile_data[tile_y * 2].bit(7 - tile_x);
                    let high = tile_data[tile_y * 2 + 1].bit(7 - tile_x);

                    let mut palette_index: u8 = 0;
                    palette_index.set_bit(0, low);
                    palette_index.set_bit(1, high);

                    if palette_index == 0 {
                        color
                    } else {
                        palette[palette_index as usize - 1]
                    }
                } else {
                    color
                }
            } else {
                color
            };

            self.hal.borrow_mut().put_pixel(line, x, color);
        }
    }
}
