
use array::Array;
use cancellation_token::{CancellationToken, TokenState};

use crate::{
    conv::Conv,
    sync_stream::{SyncReadStream, SyncWriteStream},
    FlushError,
    FlushResult,
    ReadError,
    ReadResult,
    WriteError,
    WriteResult};

pub struct InMemoryStream {
    pages: Vec<Array<u8>>,
    buffer_size: i32,
    start_idx: i32,
    end_idx: i32,
}

impl InMemoryStream {
    pub(super) fn new(buffer_size: i32) -> Self {
        Self {
            pages: Vec::new(),
            buffer_size: buffer_size,
            start_idx: 0,
            end_idx: 0,
        }
    }
}

impl InMemoryStream {    
    #[inline(always)]
    const fn max_buffer_size() -> usize { (i32::MAX - 2048) as usize }

    #[inline(always)]
    fn get_page_for_idx_mut<T: Conv<usize>>(&mut self, idx: &T) -> &mut Array<u8> {
        let buffer_size = self.buffer_size.convert();
        let page_idx = idx.convert() / buffer_size;
        if page_idx >= self.pages.len() {
            self.pages.resize_with(page_idx + 1, || { Array::new(buffer_size) });
        }
        &mut self.pages[page_idx]
    }
}


impl SyncReadStream for InMemoryStream {
    fn max_read_size() -> usize { Self::max_buffer_size() }

    fn read_with_cancellation(&mut self, buffer: &mut [u8], ct: &mut CancellationToken)
        -> Result<ReadResult, ReadError>
    {
        let buffer_len = buffer.len();
        if buffer_len == 0 {
            return Ok(ReadResult::new(0));
        }
        else if buffer_len > Self::max_read_size() {
            return Err(ReadError::OutputBufferTooBig);
        }

        let mut start = self.start_idx.convert();
        let end = self.end_idx.convert();
        if start == end {
            return Ok(ReadResult::new(0));
        }

        let buffer_size = self.buffer_size.convert();
        let mut view = buffer;
        let mut total_read = 0;

        loop {
            let in_page_idx = start % buffer_size;
            let to_read = core::cmp::min(
                buffer_size - in_page_idx,
                core::cmp::min(
                    end - start,
                    view.len()));
            
            if to_read == 0 {
                break;
            }
            total_read += to_read;

            let page = self.get_page_for_idx_mut(&start);
            let page_slice = &page.as_slice()[in_page_idx..(in_page_idx + to_read)];
            let view_slice = &mut view[0..to_read];
            view_slice.copy_from_slice(page_slice);
            start += to_read;
            if ct.get_state() == TokenState::IsCancelled {
                return Err(ReadError::IsCancelled);
            }
            let view_len = view.len();
            view = &mut view[to_read..view_len];
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        {
            self.start_idx = start as i32;
        }

        return Ok(ReadResult::new(total_read));
    }
}

impl SyncWriteStream for InMemoryStream {
    fn max_write_size() -> usize { Self::max_buffer_size() }

    fn write_with_cancellation(&mut self, buffer: &[u8], ct: &mut CancellationToken)
        -> Result<WriteResult, WriteError>
    {
        let buffer_len = buffer.len();
        if buffer_len == 0 {
            return Ok(WriteResult::new());
        }
        else if buffer_len >= Self::max_write_size() {
            return Err(WriteError::InputBufferTooBig);
        }

        let buffer_size = self.buffer_size.convert();
        let mut end = self.end_idx.convert();
        let mut view = buffer;

        loop {
            let page = self.get_page_for_idx_mut(&end);
            let in_page_idx = end % buffer_size;
            let to_write = core::cmp::min(
                buffer_size - in_page_idx,
                view.len());
            
            if to_write == 0 {
                break;
            }
            
            let page_slice = &mut page.as_slice_mut()[in_page_idx..(in_page_idx + to_write)];
            let view_slice = &view[0..to_write];
            page_slice.copy_from_slice(view_slice);
            end += to_write;
            if ct.get_state() == TokenState::IsCancelled {
                return Err(WriteError::IsCancelled);
            }
            let view_len = view.len();
            view = &view[to_write..view_len];
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        {
            self.end_idx = end as i32;
        }

        return Ok(WriteResult::new());
    }
    
    fn flush_with_cancellation(&mut self, _ct: &mut CancellationToken)
        -> Result<FlushResult, FlushError>
    {
        Ok(FlushResult::new())
    }
}


#[cfg(test)]
impl core::fmt::Debug for InMemoryStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InMemoryStream")
            .field("linked_buffer_size", &self.pages.len())
            .field("buffer_size", &self.buffer_size)
            .field("start_idx", &self.start_idx)
            .field("end_idx", &self.end_idx)
            .finish()
    }
}
