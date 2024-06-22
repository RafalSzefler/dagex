#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GenericError {
    error_code: i32,
    message: String,
}

impl GenericError {
    #[inline(always)]
    pub fn new(error_code: i32, message: String) -> Self {
        Self { error_code, message }
    }

    #[inline(always)]
    pub fn error_code(&self) -> i32 { self.error_code }

    #[inline(always)]
    pub fn message(&self) -> &String { &self.message }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ReadError {
    StreamClosed,
    InvalidOutputBuffer,
    Generic(GenericError),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum WriteError {
    StreamClosed,
    InvalidInputBuffer,
    Generic(GenericError),
}
