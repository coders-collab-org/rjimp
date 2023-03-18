pub mod bitmap;
pub mod color;
pub mod error;
pub mod handler;
pub mod plugins;
pub mod prelude;
pub mod rjimp;

pub use color::{Color, ColorRGBA};
pub use error::*;
pub use handler::ImageHandler;

#[cfg(feature = "png")]
pub use handler::PNGHandler;

#[cfg(feature = "jpeg")]
pub use handler::JPEGHandler;
pub use rjimp::RJimp;

pub type Result<T, H> = std::result::Result<T, Error<H>>;

#[cfg(feature = "png")]
pub type PNG = RJimp<PNGHandler>;
#[cfg(feature = "jpeg")]
pub type JPEG = RJimp<JPEGHandler>;
