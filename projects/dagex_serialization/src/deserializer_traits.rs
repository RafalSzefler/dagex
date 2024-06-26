use std::
    io::{Error, Read};

pub struct OwnedReadResult<T> {
    pub item: T,
    pub read_bytes: usize,
}

pub struct ReadResult<T> {
    owned: OwnedReadResult<T>,
}

impl<T> ReadResult<T> {
    pub fn new(item: T, read_bytes: usize) -> Self {
        Self { owned: OwnedReadResult { item, read_bytes } }
    }

    #[inline(always)]
    pub fn read_bytes(&self) -> usize { self.owned.read_bytes }

    #[inline(always)]
    pub fn item(&self) -> &T { &self.owned.item }

    #[inline(always)]
    pub fn release(self) -> OwnedReadResult<T> { self.owned }
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

pub trait Deserializable<T> : Sized {
    /// Deserializes Self from underlying stream.
    /// 
    /// # Errors
    /// * [`ReadError::InvalidContent`] when underlying stream cannot be 
    /// deserialized into valid object. Contains message with concrete error.
    /// * [`ReadError::IoError`] when reading from internal stream fails.
    fn deserialize(deserializer: &mut T)
        -> Result<ReadResult<Self>, ReadError>;
}

pub trait Deserializer<TRead: Read> : Sized {
    fn from_stream(stream: TRead) -> Self;

    fn release(self) -> TRead;

    /// Deserializes item from underlying stream.
    /// 
    /// # Errors
    /// * [`ReadError::InvalidContent`] when underlying stream cannot be 
    /// deserialized into valid object. Contains message with concrete error.
    /// * [`ReadError::IoError`] when reading from internal stream fails.
    fn read<T>(&mut self) -> Result<ReadResult<T>, ReadError>
        where T: Deserializable<Self>
    {
        T::deserialize(self)
    }
}
