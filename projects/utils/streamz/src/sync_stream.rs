use cancellation_token::CancellationToken;

use crate::{ReadError, ReadResult, WriteError, WriteResult};

pub trait SyncReadStream {
    fn read_with_cancellation(&mut self, buffer: &mut [u8], ct: &mut CancellationToken)
        -> Result<ReadResult, ReadError>;
    
    fn read(&mut self, buffer: &mut [u8]) -> Result<ReadResult, ReadError> {
        let mut ct = CancellationToken::default();
        self.read_with_cancellation(buffer, &mut ct)
    }
}

pub trait SyncWriteStream {
    fn write_with_cancellation(&mut self, buffer: &[u8], ct: &mut CancellationToken)
        -> Result<WriteResult, WriteError>;
    
    fn write(&mut self, buffer: &[u8]) -> Result<WriteResult, WriteError> {
        let mut ct = CancellationToken::default();
        self.write_with_cancellation(buffer, &mut ct)
    }
}

pub trait SyncStream: SyncReadStream + SyncWriteStream { }

impl<T: SyncReadStream + SyncWriteStream> SyncStream for T { }
