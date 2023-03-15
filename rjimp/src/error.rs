use derive_more::From;
use std::{fmt::Debug, io::Error as IoError};

#[derive(Debug, From)]
pub enum Error<T: HandlerError> {
    Io(IoError),
    Handler(T),
    InvalidImage,
}

 pub trait HandlerError: Debug {}
