use std::io::Read;

use crate::Deserializer;


pub struct BinaryDeserializer<TRead: Read> {
    stream: TRead,
}

impl<TRead: Read> BinaryDeserializer<TRead> {
    pub(crate) fn stream_mut(&mut self) -> &mut TRead {
        &mut self.stream
    }
}

impl<TRead: Read> Deserializer<TRead> for BinaryDeserializer<TRead> {
    fn from_stream(stream: TRead) -> Self {
        Self { stream }
    }

    fn release(self) -> TRead {
        self.stream
    }
}
