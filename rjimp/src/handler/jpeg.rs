use super::ImageHandler;
use crate::{bitmap::Bitmap, ColorRGBA, HandlerError, Result};
use derive_more::From;
use jpeg_decoder::{Decoder, PixelFormat};
use jpeg_encoder::{ColorType, Encoder};
use std::io::{Cursor, Write};

pub use jpeg_decoder::{Error as DecodingError, UnsupportedFeature};
pub use jpeg_encoder::EncodingError;

#[derive(Debug, From)]
pub enum JPEGError {
    Decoding(DecodingError),
    Encoding(EncodingError),
}

impl HandlerError for JPEGError {}

#[derive(Default)]
pub struct JPEGHandler {
    data: Vec<ColorRGBA>,
    width: u32,
    height: u32,
}
impl ImageHandler for JPEGHandler {
    type Err = JPEGError;

    fn get_mime() -> &'static str {
        "image/jpeg"
    }
    fn empty() -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        Ok(Self::default())
    }
    fn create(data: Vec<u8>) -> Result<Self, Self::Err>
    where
        Self: Sized,
    {
        let mut reader = Decoder::new(Cursor::new(data));
        reader.read_info().map_err(JPEGError::Decoding)?;

        let info = reader.info().unwrap();
        let data = reader
            .decode()
            .map(|pixels| match info.pixel_format {
                PixelFormat::L8 => {
                    let mut result = Vec::with_capacity(pixels.len());

                    for pixel in pixels {
                        result.push(ColorRGBA(pixel, pixel, pixel, 0xFF));
                    }

                    result
                }
                PixelFormat::L16 => {
                    let mut result = Vec::with_capacity(pixels.len() / 2);

                    for pixel in pixels.chunks_exact(2) {
                        let gray = u16::from_be_bytes([pixel[0], pixel[1]]);
                        let scaled = (gray as f32 / u16::MAX as f32 * 255.0) as u8;
                        result.push(ColorRGBA(scaled, scaled, scaled, 0xFF));
                    }

                    result
                }
                PixelFormat::RGB24 => {
                    let mut result = Vec::with_capacity(pixels.len() / 3);
                    for rgb in pixels.chunks_exact(3) {
                        result.push(ColorRGBA(rgb[0], rgb[1], rgb[2], 0xFF));
                    }

                    result
                }
                PixelFormat::CMYK32 => {
                    let mut result = Vec::with_capacity(pixels.len() / 4);

                    for pixel in pixels.chunks_exact(4) {
                        let c = pixel[0] as f32 / 255.0;
                        let m = pixel[1] as f32 / 255.0;
                        let y = pixel[2] as f32 / 255.0;
                        let k = pixel[3] as f32 / 255.0;

                        let r = 255.0 * (1.0 - c) * (1.0 - k);
                        let g = 255.0 * (1.0 - m) * (1.0 - k);
                        let b = 255.0 * (1.0 - y) * (1.0 - k);

                        result.push(ColorRGBA(r as u8, g as u8, b as u8, 0xFF));
                    }

                    result
                }
            })
            .map_err(JPEGError::Decoding)?;

        Ok(Self {
            data,
            width: info.width as u32,
            height: info.height as u32,
        })
    }

    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn get_pixel_index(&self, x: u32, y: u32) -> Option<usize> {
        let width = self.width();
        let height = self.height();

        if x >= width && y >= height {
            return None;
        }

        Some(((width * y + x) * 4) as usize)
    }

    fn get_pixel_color_by_index(&self, index: usize) -> Option<u32> {
        self.data.get(index).map(ColorRGBA::to_int)
    }

    fn plugin<T, O>(&mut self, plugin: T, options: O)
    where
        T: FnOnce(O, Bitmap),
    {
        plugin(options, self.bitmap())
    }

    fn encode(&self, writer: impl Write) -> Result<(), Self::Err>
    where
        Self: Sized,
    {
        let encoder = Encoder::new(writer, 100);

        encoder
            .encode(
                &self
                    .data
                    .iter()
                    .flat_map(|p| p.to_bytes())
                    .collect::<Vec<u8>>(),
                self.width as u16,
                self.height as u16,
                ColorType::Rgba,
            )
            .map_err(JPEGError::Encoding)?;

        Ok(())
    }

    #[inline]
    fn bitmap(&mut self) -> Bitmap {
        Bitmap {
            width: self.width(),
            height: self.height(),
            data: &mut self.data,
        }
    }
}
