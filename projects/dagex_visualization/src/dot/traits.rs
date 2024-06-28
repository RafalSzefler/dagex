use std::io::{Error, Write};

pub trait DotSerializable<TWrite, TSer>
    where TWrite : Write,
        TSer: DotSerializer<TWrite>
{
    /// Serializes the item into Serializer.  On Ok returns number of
    /// written bytes.
    /// 
    /// # Errors
    /// Forwards errors from the underlying stream.
    fn serialize(&self, ser: &mut TSer) -> Result<usize, Error>;
}

pub trait DotSerializer<TWrite: Write> : From<TWrite> {
    /// Releases the underlying stream.
    fn release(self) -> TWrite;

    /// Serializes graph into underlying stream. On Ok returns number of
    /// written bytes.
    /// 
    /// # Errors
    /// Forwards errors from the underlying stream.
    fn serialize<T>(&mut self, item: &T) -> Result<usize, Error>
        where T: DotSerializable<TWrite, Self>
    {
        item.serialize(self)
    }
}
