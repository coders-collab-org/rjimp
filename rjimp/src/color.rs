#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8),
}

impl Color {
    #[inline]
    pub fn red() -> Self {
        Color::Rgba(0xFF, 0x00, 0x00, 0xFF)
    }

    #[inline]
    pub fn green() -> Self {
        Color::Rgba(0x00, 0xFF, 0x00, 0xFF)
    }

    #[inline]
    pub fn blue() -> Self {
        Color::Rgba(0x00, 0x00, 0xFF, 0xFF)
    }

    #[inline]
    pub fn as_hex(&self) -> u32 {
        u32::from_be_bytes(self.as_rgba())
    }

    #[inline]
    pub fn as_rgba(&self) -> [u8; 4] {
        match *self {
            Color::Rgb(r, g, b) => [r, g, b, 0xFF],
            Color::Rgba(r, g, b, a) => [r, g, b, a],
        }
    }
}
