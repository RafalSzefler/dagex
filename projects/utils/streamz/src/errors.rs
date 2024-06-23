#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GenericError {
    error_code: i32,
    message: String,
    stream_id: String,
}

impl GenericError {
    #[inline(always)]
    pub fn new(error_code: i32, message: String, stream_id: String) -> Self {
        Self { error_code, message, stream_id }
    }

    #[inline(always)]
    pub fn error_code(&self) -> i32 { self.error_code }

    #[inline(always)]
    pub fn message(&self) -> &String { &self.message }

    #[inline(always)]
    pub fn stream_id(&self) -> &String { &self.message }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FlushError {
    /// Stream is closed.
    StreamClosed,

    /// Flush operation has been cancelled.
    IsCancelled,

    /// Stream specific generic error.
    Generic(GenericError),
}
