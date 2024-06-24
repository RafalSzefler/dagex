use super::{defaults::{DEFAULT_BUFFER_SIZE, MAX_BUFFER_SIZE}, InMemoryStream};

#[derive(Debug)]
pub enum InMemoryStreamBuildError {
    /// Passed buffer bigger than [`InMemoryStreamBuilder::max_buffer_size()`].
    BufferSizeTooSmall,

    /// Passed buffer smaller than [`InMemoryStreamBuilder::min_buffer_size()`].
    BufferSizeTooBig,
}

pub struct InMemoryStreamBuilder {
    buffer_size: usize,
}

impl InMemoryStreamBuilder {
    pub const fn max_buffer_size() -> usize { MAX_BUFFER_SIZE }

    pub const fn min_buffer_size() -> usize { 2 }

    pub fn set_buffer_size(&mut self, value: usize) {
        self.buffer_size = value;
    }

    /// Builds [`InMemoryStream`].
    /// 
    /// # Errors
    /// For the description of errors see [`InMemoryStreamBuildError`] docs.
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    pub fn build(self) -> Result<InMemoryStream, InMemoryStreamBuildError> {
        if self.buffer_size < Self::min_buffer_size() {
            return Err(InMemoryStreamBuildError::BufferSizeTooSmall);
        }

        if self.buffer_size > Self::max_buffer_size() {
            return Err(InMemoryStreamBuildError::BufferSizeTooBig);
        }

        Ok(InMemoryStream::new(self.buffer_size as i32))
    }
}

impl Default for InMemoryStreamBuilder {
    fn default() -> Self {
        Self { buffer_size: DEFAULT_BUFFER_SIZE }
    }
}


#[cfg(test)]
impl core::fmt::Debug for InMemoryStreamBuilder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InMemoryStreamBuilder")
            .field("buffer_size", &self.buffer_size)
            .finish()
    }
}
