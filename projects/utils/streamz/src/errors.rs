use immutable_string::ImmutableString;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GenericError {
    error_code: Option<i32>,
    message: ImmutableString,
    stream_id: ImmutableString,
}

impl GenericError {
    #[inline(always)]
    pub fn new(error_code: Option<i32>, message: ImmutableString, stream_id: ImmutableString) -> Self {
        Self { error_code, message, stream_id }
    }

    #[inline(always)]
    pub fn error_code(&self) -> Option<i32> { self.error_code }

    #[inline(always)]
    pub fn message(&self) -> &ImmutableString { &self.message }

    #[inline(always)]
    pub fn stream_id(&self) -> &ImmutableString { &self.stream_id }
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
