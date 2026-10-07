#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub [u8; 4]);

impl Default for Color {
    fn default() -> Self {
        Color::WHITE
    }
}

impl Color {
    pub const fn hex(rgba: u32) -> Self {
        Color(rgba.to_be_bytes())
    }

    pub const RED: Self = Color([255, 0, 0, 255]);
    pub const GREEN: Self = Color([0, 255, 0, 255]);
    pub const BLUE: Self = Color([0, 0, 255, 255]);
    pub const WHITE: Self = Color([255, 255, 255, 255]);
    pub const BLACK: Self = Color([0, 0, 0, 255]);
    pub const TRANSPARENT: Self = Color([0, 0, 0, 0]);

    // https://coolors.co/11151c-1f2633-677ba2-f98510
    pub const BACKGROUND: Self = Color::hex(0x11151Cff);
    pub const BACKGROUND_ALT: Self = Color::hex(0x1F2633ff);
    pub const PRIMARY: Self = Color::hex(0x677BA2ff);
    pub const ACCENT: Self = Color::hex(0xF98510ff);
}
