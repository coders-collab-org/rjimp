use crate::bitmap::Bitmap;

#[derive(Debug, Default, Clone)]
pub struct CircleOptions {
    pub radius: Option<u32>,
    pub x: Option<u32>,
    pub y: Option<u32>,
}

pub fn circle(options: CircleOptions, bitmap: Bitmap) {
    let w = bitmap.width;
    let h = bitmap.height;

    let radius = options.radius.map_or(if w > h { h } else { w } / 2, |r| r) as f32;
    let center_x = options.x.map_or(w / 2, |x| x) as f32;
    let center_y = options.y.map_or(h / 2, |x| x) as f32;

    for (x, y, idx) in bitmap.scan(0, 0) {
        let current_radius = ((x as f32 - center_x).powi(2) + (y as f32 - center_y).powi(2)).sqrt();
        let diff = radius - current_radius;

        if diff <= 0.0 {
            bitmap.data[idx + 3] = 0;
        } else if diff < 1.0 {
            bitmap.data[idx + 3] = (255.0 * diff) as u8;
        }
    }
}
