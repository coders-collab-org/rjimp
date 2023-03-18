#[cfg(feature = "jpeg")]
mod jpeg;
#[cfg(feature = "png")]
mod png;

#[cfg(feature = "jpeg")]
pub use self::jpeg::*;
#[cfg(feature = "png")]
pub use self::png::*;

use std::io::Write;

use crate::{bitmap::Bitmap, HandlerError, Result};

pub trait ImageHandler {
    type Err: HandlerError;

    fn create(data: Vec<u8>) -> Result<Self, Self::Err>
    where
        Self: Sized;
    fn empty() -> Result<Self, Self::Err>
    where
        Self: Sized;
    fn get_mime() -> &'static str;
    fn encode(&self, writer: impl Write) -> Result<(), Self::Err>
    where
        Self: Sized;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn get_pixel_index(&self, x: u32, y: u32) -> Option<usize>;
    fn get_pixel_color_by_index(&self, index: usize) -> Option<u32>;

    #[inline]
    fn get_pixel_hex_color_by_index(&self, index: usize) -> Option<String> {
        self.get_pixel_color_by_index(index)
            .map(|c| format!("#{c:X}"))
    }

    #[inline]
    fn get_pixel_color(&self, x: u32, y: u32) -> Option<u32> {
        self.get_pixel_color_by_index(self.get_pixel_index(x, y)?)
    }

    #[inline]
    fn get_pixel_hex_color(&self, x: u32, y: u32) -> Option<String> {
        self.get_pixel_hex_color_by_index(self.get_pixel_index(x, y)?)
    }

    fn bitmap(&mut self) -> Bitmap;

    fn plugin<T, O>(&mut self, plugin: T, options: O)
    where
        T: FnOnce(O, Bitmap);
}
