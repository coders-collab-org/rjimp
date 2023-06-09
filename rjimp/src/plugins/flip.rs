use crate::{bitmap::Bitmap, ColorRGBA};

#[derive(Debug, Clone)]
pub struct FlipOptions {
    pub horizontal: bool,
    pub vertical: bool,
}

impl Default for FlipOptions {
    fn default() -> Self {
        Self {
            horizontal: true,
            vertical: false,
        }
    }
}

pub fn flip(options: FlipOptions, bitmap: Bitmap) {
    let mut temp = vec![ColorRGBA::default(); bitmap.data.len()];

    for (x, y, idx) in bitmap.scan(0, 0) {
        let moved_x = if options.horizontal {
            bitmap.width - 1 - x
        } else {
            x
        };

        let moved_y = if options.vertical {
            bitmap.height - 1 - y
        } else {
            y
        };

        let moved_idx = (bitmap.width * moved_y + moved_x) as usize;

        temp[moved_idx] = bitmap.data[idx];
    }

    *bitmap.data = temp;
}
