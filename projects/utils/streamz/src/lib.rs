#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
pub(crate) mod conv;
mod errors;
mod results;

pub use errors::{ReadError, WriteError, FlushError};
pub use results::{ReadResult, WriteResult, FlushResult};

pub mod sync_stream;
pub mod concrete;
