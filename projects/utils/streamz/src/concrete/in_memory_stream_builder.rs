use super::InMemoryStream;

pub const DEFAULT_BUFFER_SIZE: i32 = 1 << 14;


#[derive(Debug)]
pub enum InMemoryStreamBuildError {
    BufferSizeTooSmall,
    BufferSizeTooBig,
}

pub struct InMemoryStreamBuilder {
    buffer_size: i32,
}

impl InMemoryStreamBuilder {
    pub fn set_buffer_size(&mut self, value: i32) {
        self.buffer_size = value;
    }

    pub fn build(self) -> Result<InMemoryStream, InMemoryStreamBuildError> {
        if self.buffer_size < InMemoryStream::min_buffer_size() {
            return Err(InMemoryStreamBuildError::BufferSizeTooSmall);
        }

        if self.buffer_size > InMemoryStream::max_buffer_size() {
            return Err(InMemoryStreamBuildError::BufferSizeTooBig);
        }

        Ok(InMemoryStream::new(self.buffer_size))
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
