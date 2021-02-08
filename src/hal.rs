use std::convert::TryInto;

#[derive(Copy, Clone)]
pub enum Color {
    White = 0,
    LightGrey = 1,
    DarkGrey = 2,
    Black = 3,
}

impl Color {
    pub fn into_rgba(self) -> [u8; 4] {
        match self {
            Color::White => [155, 188, 15, 255],
            Color::LightGrey => [139, 172, 15, 255],
            Color::DarkGrey => [48, 98, 48, 255],
            Color::Black => [15, 56, 15, 255],
        }
    }

    pub fn into_rgba_f32(self) -> [f32; 4] {
        self.into_rgba()
            .iter()
            .map(|&c| c as f32 / u8::MAX as f32)
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap()
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Joypad {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

pub trait HAL {
    fn is_joypad_pressed(&self, button: Joypad) -> bool;
    fn put_pixel(&mut self, line: usize, x: usize, color: Color);
    fn serial_callback(&mut self, value: u8) -> u8;
}
