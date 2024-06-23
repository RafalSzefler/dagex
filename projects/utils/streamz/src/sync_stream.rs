use cancellation_token::CancellationToken;

use crate::{errors::FlushError, results::FlushResult, ReadError, ReadResult, WriteError, WriteResult};

pub trait SyncReadStream {
    /// Returns maximum buffer size for read operations.
    fn max_read_size() -> usize;

    /// Reads data into buffer. [`ReadResult`] contains number of bytes read.
    /// 
    /// # Errors
    /// For the description of errors see [`ReadError`] docs.
    fn read_with_cancellation(&mut self, buffer: &mut [u8], ct: &mut CancellationToken)
        -> Result<ReadResult, ReadError>;
    
    /// Reads data into buffer. [`ReadResult`] contains number of bytes read.
    /// Unlike [`SyncReadStream::read_with_cancellation`] this operation cannot be
    /// cancelled.
    /// 
    /// # Errors
    /// For the description of errors see [`ReadError`] docs.
    fn read(&mut self, buffer: &mut [u8]) -> Result<ReadResult, ReadError> {
        let mut ct = CancellationToken::default();
        self.read_with_cancellation(buffer, &mut ct)
    }
}

pub trait SyncWriteStream {
    /// Returns maximum buffer size for write operations.
    fn max_write_size() -> usize;

    /// Writes entire buffer into stream. On success returns [`WriteResult`].
    /// 
    /// # Errors
    /// For the description of errors see [`WriteError`] docs.
    fn write_with_cancellation(&mut self, buffer: &[u8], ct: &mut CancellationToken)
        -> Result<WriteResult, WriteError>;
    
    /// Flushes straem. On success returns [`FlushResult`].
    /// 
    /// # Errors
    /// For the description of errors see [`FlushError`] docs.
    fn flush_with_cancellation(&mut self, ct: &mut CancellationToken)
        -> Result<FlushResult, FlushError>;
    
    /// Writes entire buffer into stream. On success returns [`WriteResult`].
    /// Unlike [`SyncWriteStream::write_with_cancellation`] cannot be cancelled.
    /// 
    /// # Errors
    /// For the description of errors see [`WriteError`] docs.
    fn write(&mut self, buffer: &[u8]) -> Result<WriteResult, WriteError> {
        let mut ct = CancellationToken::default();
        self.write_with_cancellation(buffer, &mut ct)
    }

    /// Flushes straem. On success returns [`FlushResult`]. Unlike
    /// [`SyncWriteStream::flush_with_cancellation`] cannot be cancelled.
    /// 
    /// # Errors
    /// For the description of errors see [`FlushError`] docs.
    fn flush(&mut self) -> Result<FlushResult, FlushError> {
        let mut ct = CancellationToken::default();
        self.flush_with_cancellation(&mut ct)
    }
}

pub trait SyncStream: SyncReadStream + SyncWriteStream { }

impl<T: SyncReadStream + SyncWriteStream> SyncStream for T { }
