use std::fs::File;

use super::{defaults::DEFAULT_BUFFER_SIZE, FileStream};

#[derive(Debug)]
pub enum FileStreamBuildError {
    BufferSizeTooSmall,
    BufferSizeTooBig,
    FileDoesNotExist,
    FileNotSet,
}

pub struct FileStreamBuilder {
    file: Option<File>,
    buffer_size: Option<usize>,
}

impl FileStreamBuilder {
    pub fn set_buffer_size(&mut self, size: usize) {
        self.buffer_size = Some(size);
    }

    pub fn no_buffer(&mut self) {
        self.buffer_size = None;
    }

    pub fn set_file(&mut self, file: File) {
        self.file = Some(file);
    }
    
    pub fn build(self) -> Result<FileStream, FileStreamBuildError> {
        let file: File;
        match self.file {
            Some(local_file) => {
                file = local_file;
            },
            None => {
                return Err(FileStreamBuildError::FileNotSet);
            }
        }

        Ok(FileStream::new(Some(file)))
    }
}

impl Default for FileStreamBuilder {
    fn default() -> Self {
        Self { file: None, buffer_size: Some(DEFAULT_BUFFER_SIZE) }
    }
}
