
use array::Array;
use cancellation_token::CancellationToken;

use crate::{
    sync_stream::{SyncReadStream, SyncWriteStream},
    ReadError,
    ReadResult,
    WriteError,
    WriteResult};

pub struct InMemoryStream {
    linked_buffer: Vec<Array<u8>>,
    buffer_size: i32,
    start_idx: i32,
    end_idx: i32,
}

impl InMemoryStream {
    pub const fn max_buffer_size() -> i32 { i32::MAX - 1024 }

    pub const fn min_buffer_size() -> i32 { 8 }

    pub(super) fn new(buffer_size: i32) -> Self {
        Self {
            linked_buffer: Vec::new(),
            buffer_size: buffer_size,
            start_idx: 0,
            end_idx: 0,
        }
    }
}

impl SyncReadStream for InMemoryStream {
    fn read_with_cancellation(&mut self, buffer: &mut [u8], _ct: &mut CancellationToken)
        -> Result<ReadResult, ReadError>
    {
        todo!()
    }
}

impl SyncWriteStream for InMemoryStream {
    fn write_with_cancellation(&mut self, buffer: &[u8], _ct: &mut CancellationToken)
        -> Result<WriteResult, WriteError>
    {
        todo!()
    }
}


#[cfg(test)]
impl core::fmt::Debug for InMemoryStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InMemoryStream")
            .field("linked_buffer", &self.linked_buffer)
            .field("buffer_size", &self.buffer_size)
            .field("start_idx", &self.start_idx)
            .field("end_idx", &self.end_idx)
            .finish()
    }
}
