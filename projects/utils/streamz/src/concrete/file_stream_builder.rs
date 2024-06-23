#[derive(Debug)]
pub enum FileStreamBuildError {
    BufferSizeTooSmall,
    BufferSizeTooBig,
    FileDoesNotExist,
}

pub struct FileStreamBuilder {
    
}
