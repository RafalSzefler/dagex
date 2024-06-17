#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod callable_with_flag;
mod errors;
mod traits;
mod core;

pub(crate) use callable_with_flag::CallableWithFlag;
pub use errors::IsCancelled;
pub use traits::{CancellationTokenRegistration, CancellationToken, CancellationTokenSource};

