use std::{
    io::{Error, Write},
    marker::PhantomData};

use crate::WithTypeInfo;

pub struct WriteResult<T> {
    written_bytes: usize,
    phantom: PhantomData<T>,
}

impl<T> WriteResult<T> {
    pub fn new(written_bytes: usize) -> Self {
        Self { written_bytes, phantom: PhantomData }
    }

    pub fn written_bytes(&self) -> usize { self.written_bytes }
}

#[derive(Debug)]
pub enum WriteError {
    IoError(Error),
}

impl From<Error> for WriteError {
    fn from(value: Error) -> Self {
        WriteError::IoError(value)
    }
}

pub trait Serializer<TWrite: Write> {
    fn from_stream(stream: TWrite) -> Self;

    fn release(self) -> TWrite;

    /// Serializes item into underlying stream.
    /// 
    /// # Errors
    /// In case the underlying stream fails, returns that error
    /// embedded in [`WriteError`].
    fn write<T>(&mut self, item: &T)
        -> Result<WriteResult<T>, WriteError>
        where T: WithTypeInfo;
}
