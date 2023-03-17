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
        u32::from_be_bytes(self.as_rgba().to_bytes())
    }

    #[inline]
    pub fn as_rgba(&self) -> ColorRGBA {
        match *self {
            Color::Rgb(r, g, b) => ColorRGBA(r, g, b, 0xFF),
            Color::Rgba(r, g, b, a) => ColorRGBA(r, g, b, a),
        }
    }
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub struct ColorRGBA(pub u8, pub u8, pub u8, pub u8);

impl ColorRGBA {
    #[inline]
    pub fn to_bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    #[inline]
    pub fn to_int(&self) -> u32 {
        u32::from_be_bytes(self.to_bytes())
    }
}
