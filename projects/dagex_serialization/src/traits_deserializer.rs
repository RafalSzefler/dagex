use std::
    io::{Error, Read};

use crate::WithTypeInfo;

pub struct ReadResult<T> {
    item: T,
    read_bytes: usize,
}

impl<T> ReadResult<T> {
    pub fn new(item: T, read_bytes: usize) -> Self {
        Self { item, read_bytes }
    }

    #[inline(always)]
    pub fn read_bytes(&self) -> usize { self.read_bytes }

    #[inline(always)]
    pub fn release(self) -> T { self.item }
}

#[derive(Debug)]
pub enum ReadError {
    InvalidContent(String),
    IoError(Error),
}

impl From<Error> for ReadError {
    fn from(value: Error) -> Self {
        ReadError::IoError(value)
    }
}

pub trait Deserializer<TRead: Read> {
    fn from_stream(stream: TRead) -> Self;

    fn release(self) -> TRead;

    /// Deserializes item from underlying stream.
    /// 
    /// # Errors
    /// * [`ReadError::InvalidContent`] when underlying stream cannot be 
    /// deserialized into valid object. Contains message with concrete error.
    /// * [`ReadError::IoError`] when reading from internal stream fails.
    fn read<T>(&mut self) -> Result<ReadResult<T>, ReadError>
        where T: WithTypeInfo;
}
