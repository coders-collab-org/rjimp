use crate::{handler::ImageHandler, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use std::{
    fs::File,
    io::{BufWriter, Cursor},
    ops::{Deref, DerefMut},
};
use tokio::fs;

pub struct RJimp<T: ImageHandler> {
    handler: T,
}

impl<T: ImageHandler> RJimp<T> {
    pub async fn new(path: &str) -> Result<Self, T::Err> {
        let data = fs::read(path).await?;

        Self::from_vec(data)
    }

    #[inline]
    pub fn from_vec(data: Vec<u8>) -> Result<Self, T::Err> {
        return Ok(T::create(data)?.into());
    }

    #[inline]
    pub fn write(&self, path: &str) -> Result<(), T::Err> {
        let file = File::create(path)?;

        let writer = BufWriter::new(file);

        self.handler.encode(writer)?;

        Ok(())
    }

    #[inline]
    pub fn get_buffer64(&self) -> Result<String, T::Err> {
        let mut img_data = vec![];

        self.handler.encode(Cursor::new(&mut img_data))?;

        Ok(format!(
            "data:{};base64,{}",
            T::get_mime(),
            STANDARD.encode(&img_data)
        ))
    }
}

impl<T: ImageHandler> Deref for RJimp<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.handler
    }
}
impl<T: ImageHandler> DerefMut for RJimp<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.handler
    }
}

impl<T: ImageHandler> From<T> for RJimp<T> {
    fn from(handler: T) -> Self {
        Self { handler }
    }
}

impl<T: ImageHandler + Clone> Clone for RJimp<T> {
    fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone(),
        }
    }
}
