use std::io::Write;

use crate::Serializer;

pub struct BinarySerializer<TWrite: Write> {
    stream: TWrite,
}

impl<TWrite: Write> BinarySerializer<TWrite> {
    pub(crate) fn stream_mut(&mut self) -> &mut TWrite {
        &mut self.stream
    }
}

impl<TWrite: Write> Serializer<TWrite> for BinarySerializer<TWrite> {
    fn from_stream(stream: TWrite) -> Self {
        Self { stream }
    }

    fn release(self) -> TWrite {
        self.stream
    }
}
