use super::ImageHandler;
use crate::{bitmap::Bitmap, ColorRGBA, Error, HandlerError, Result};
use derive_more::From;
use png::{ColorType, Decoder, DecodingError, Encoder, EncodingError};
use std::io::{Cursor, Write};

#[derive(Debug, From)]
pub enum PNGError {
    Decoding(DecodingError),
    Encoding(EncodingError),
}

impl HandlerError for PNGError {}

#[derive(Default)]
pub struct PNGHandler {
    data: Vec<ColorRGBA>,
    width: u32,
    height: u32,
}
impl ImageHandler for PNGHandler {
    type Err = PNGError;

    fn get_mime() -> &'static str {
        "image/png"
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
        let mut reader = Decoder::new(Cursor::new(data))
            .read_info()
            .map_err(PNGError::Decoding)?;

        if reader.info().is_animated() {
            return Err(Error::InvalidImage);
        }
        let mut bytes = vec![0; reader.output_buffer_size()];

        let info = reader.next_frame(&mut bytes).map_err(PNGError::Decoding)?;

        let data = match info.color_type {
            ColorType::Grayscale | ColorType::Indexed => {
                let mut result = Vec::with_capacity(bytes.len());

                for mut byte in bytes {
                    if info.color_type == ColorType::Indexed {
                        byte = *reader
                            .info()
                            .palette
                            .as_ref()
                            .and_then(|p| p.get(byte as usize))
                            .ok_or_else(|| Error::InvalidImage)?;
                    }
                    result.push(ColorRGBA(byte, byte, byte, 0xFF));
                }

                result
            }
            ColorType::Rgb => {
                let mut result = Vec::with_capacity(bytes.len() / 3);
                for rgb in bytes.windows(3) {
                    result.push(ColorRGBA(rgb[0], rgb[1], rgb[2], 0xFF));
                }

                result
            }
            ColorType::GrayscaleAlpha => {
                let mut result = Vec::with_capacity(bytes.len() / 2);
                for ga in bytes.windows(2) {
                    result.push(ColorRGBA(ga[0], ga[0], ga[0], ga[1]));
                }

                result
            }
            _ => {
                let mut result = Vec::with_capacity(bytes.len() / 4);
                for rgba in bytes.windows(4) {
                    result.push(ColorRGBA(rgba[0], rgba[1], rgba[2], rgba[3]));
                }

                result
            }
        };

        Ok(Self {
            data,
            width: info.width,
            height: info.height,
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

        Some((width * y + x) as usize)
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
        let mut encoder = Encoder::new(writer, self.width(), self.width());

        encoder.set_color(ColorType::Rgba);

        let mut writer = encoder.write_header().map_err(PNGError::Encoding)?;

        writer
            .write_image_data(
                &self
                    .data
                    .iter()
                    .flat_map(ColorRGBA::to_bytes)
                    .collect::<Vec<u8>>(),
            )
            .map_err(PNGError::Encoding)?;

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
