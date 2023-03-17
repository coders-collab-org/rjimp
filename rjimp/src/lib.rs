pub mod bitmap;
pub mod error;
pub mod handler;
#[macro_use]
pub mod plugins;
pub mod color;
pub mod prelude;
pub mod rjimp;

pub use color::Color;
pub use color::ColorRGBA;
pub use error::*;
pub use handler::ImageHandler;
pub use handler::PNGHandler;
pub use rjimp::RJimp;

pub type Result<T, H> = std::result::Result<T, Error<H>>;
pub type PNG = RJimp<PNGHandler>;
