use std::{fs::File, io::{Read, Write}, sync::OnceLock};

use cancellation_token::{CancellationToken, TokenState};
use immutable_string::ImmutableString;

use crate::{
    errors::GenericError, sync_stream::{SyncReadStream, SyncWriteStream}, FlushError, FlushResult, ReadError, ReadResult, WriteError, WriteResult};

use super::defaults::{DEFAULT_BUFFER_SIZE, MAX_BUFFER_SIZE};

pub struct FileStream {
    file: Option<File>,
}

fn get_stream_id() -> ImmutableString {
    static STREAM_ID: OnceLock<ImmutableString> = OnceLock::new();
    STREAM_ID.get_or_init(|| { ImmutableString::get("FileStream").unwrap() }).clone()
}

fn build_generic_read_error(error_code: Option<i32>, message: &str) -> ReadError {
    let immutable_message = ImmutableString::get(message).unwrap();
    ReadError::Generic(GenericError::new(error_code, immutable_message, get_stream_id()))
}

fn build_generic_write_error(error_code: Option<i32>, message: &str) -> WriteError {
    let immutable_message = ImmutableString::get(message).unwrap();
    WriteError::Generic(GenericError::new(error_code, immutable_message, get_stream_id()))
}

fn build_generic_flush_error(error_code: Option<i32>, message: &str) -> FlushError {
    let immutable_message = ImmutableString::get(message).unwrap();
    FlushError::Generic(GenericError::new(error_code, immutable_message, get_stream_id()))
}

impl FileStream {
    pub(crate) fn new(file: Option<File>) -> Self {
        Self { file }
    }

    pub fn release_file(self) -> Option<File> { self.file }
}

impl SyncReadStream for FileStream {
    fn max_read_size() -> usize { MAX_BUFFER_SIZE }

    fn read_with_cancellation(&mut self, buffer: &mut [u8], ct: &mut CancellationToken)
        -> Result<ReadResult, ReadError>
    {
        let file = match &mut self.file {
            Some(file) => file,
            None => {
                return Err(build_generic_read_error(Some(-1), "File not set."));
            },
        };

        let buffer_len = buffer.len();
        let mut total_read_bytes = 0;
        let mut view = buffer;

        while total_read_bytes < buffer_len {
            let to_read = core::cmp::min(
                buffer_len,
                core::cmp::min(view.len(), DEFAULT_BUFFER_SIZE));
            let tmp_view = &mut view[0..to_read];
            match file.read(tmp_view) {
                Ok(size) => {
                    if size == 0 {
                        if total_read_bytes == 0 {
                            return Err(ReadError::StreamClosed);
                        }
                        return Ok(ReadResult::new(total_read_bytes));
                    }
                    total_read_bytes += size;
                    let view_len = view.len();
                    view = &mut view[to_read..view_len];
                },
                Err(err) => {
                    let generic = build_generic_read_error(
                        err.raw_os_error(),
                        err.to_string().as_str());
                    return Err(generic);
                },
            }

            if ct.get_state() == TokenState::IsCancelled {
                return Err(ReadError::IsCancelled);
            }
        }

        return Ok(ReadResult::new(total_read_bytes));
    }
}

impl SyncWriteStream for FileStream {
    fn max_write_size() -> usize { MAX_BUFFER_SIZE }

    fn write_with_cancellation(&mut self, buffer: &[u8], ct: &mut CancellationToken)
        -> Result<WriteResult, WriteError>
    {
        let file = match &mut self.file {
            Some(file) => file,
            None => {
                return Err(build_generic_write_error(Some(-1), "File not set."));
            },
        };

        let buffer_len = buffer.len();
        let mut total_written_bytes = 0;

        while total_written_bytes < buffer_len {
            let to_write = core::cmp::min(
                buffer_len - total_written_bytes,
                DEFAULT_BUFFER_SIZE);
            let view = &buffer[total_written_bytes..(total_written_bytes+to_write)];
            match file.write_all(view) {
                Ok(_) => {
                    total_written_bytes += to_write;
                },
                Err(err) => {
                    let generic = build_generic_write_error(
                        err.raw_os_error(),
                        err.to_string().as_str());
                    return Err(generic);
                },
            }

            if ct.get_state() == TokenState::IsCancelled {
                return Err(WriteError::IsCancelled);
            }
        }

        Ok(WriteResult::new())
    }

    fn flush_with_cancellation(&mut self, _ct: &mut CancellationToken)
        -> Result<FlushResult, FlushError>
    {
        let file = match &mut self.file {
            Some(file) => file,
            None => {
                return Err(build_generic_flush_error(Some(-1), "File not set."));
            },
        };
        match file.flush() {
            Ok(_) => { },
            Err(err) => {
                let generic = build_generic_flush_error(
                    err.raw_os_error(),
                    err.to_string().as_str());
                return Err(generic);
            }
        }

        Ok(FlushResult::new())
    }
}