use crate::{color::ColorRGBA, Color};

pub struct Bitmap<'a> {
    pub(crate) data: &'a mut Vec<ColorRGBA>,
    pub width: u32,
    pub height: u32,
}

impl Bitmap<'_> {
    #[inline]
    pub fn scan(&self, x: u32, y: u32) -> impl Iterator<Item = (u32, u32, usize)> {
        let mut cur_x = x;
        let mut cur_y = y;
        let width = self.width;
        let height = self.height;

        std::iter::from_fn(move || {
            if cur_y >= height {
                cur_x += 1;
                cur_y = 0;

                if cur_x >= width {
                    return None;
                }
            }

            let result = Some((cur_x, cur_y, (width * cur_y + cur_x) as usize));

            cur_y += 1;

            result
        })
    }
    #[inline]
    pub fn set_pixel_by_index(&mut self, idx: usize, color: Color) -> bool {
        if let Some(pixel) = self.data.get_mut(idx) {
            *pixel = color.as_rgba();

            return true;
        };

        false
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> bool {
        self.set_pixel_by_index((self.width * y + x) as usize, color)
    }
}
