#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod errors;
mod results;

pub use errors::{ReadError, WriteError};
pub use results::{ReadResult, WriteResult};

pub mod sync_stream;
pub mod concrete;
