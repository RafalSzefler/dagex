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
    /// Stream is closed.
    StreamClosed,

    /// Passsed buffer is too big.
    OutputBufferTooBig,

    /// Read operation has been cancelled.
    IsCancelled,

    /// Stream specific generic error.
    Generic(GenericError),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum WriteError {
    /// Stream is closed.
    StreamClosed,

    /// Passsed buffer is too big.
    InputBufferTooBig,

    /// Write operation has been cancelled.
    IsCancelled,

    /// Stream specific generic error.
    Generic(GenericError),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FlushError {
    /// Stream is closed.
    StreamClosed,

    /// Flush operation has been cancelled.
    IsCancelled,

    /// Stream specific generic error.
    Generic(GenericError),
}
