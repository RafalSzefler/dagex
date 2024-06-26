use std::{
    io::{Error, Write},
    marker::PhantomData};

pub struct OwnedWriteResult {
    pub written_bytes: usize,
}

pub struct WriteResult<T> {
    owned: OwnedWriteResult,
    phantom: PhantomData<T>,
}

impl<T> WriteResult<T> {
    pub fn new(written_bytes: usize) -> Self {
        Self { owned: OwnedWriteResult { written_bytes }, phantom: PhantomData }
    }

    pub fn written_bytes(&self) -> usize { self.owned.written_bytes }

    pub fn release(self) -> OwnedWriteResult { self.owned }
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

pub trait Serializable<T> : Sized {
    /// Serializes self through underlying erializer.
    /// 
    /// # Errors
    /// In case the underlying stream fails, returns that error
    /// embedded in [`WriteError`].
    fn serialize(&self, serializer: &mut T)
        -> Result<WriteResult<Self>, WriteError>;
}

pub trait Serializer<TWrite: Write> : Sized {
    fn from_stream(stream: TWrite) -> Self;

    fn release(self) -> TWrite;

    /// Serializes item into underlying stream.
    /// 
    /// # Errors
    /// In case the underlying stream fails, returns that error
    /// embedded in [`WriteError`].
    fn write<T>(&mut self, item: &T) -> Result<WriteResult<T>, WriteError>
        where T: Serializable<Self>
    {
        item.serialize(self)
    }
}
