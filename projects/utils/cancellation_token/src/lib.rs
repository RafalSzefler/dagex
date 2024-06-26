#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
pub(crate) mod pdi;
pub(crate) mod callable;
mod token_state;
pub(crate) mod cancellation_token_inner;
mod cancellation_token;

pub use token_state::TokenState;
pub use cancellation_token::{
    CancellationTokenSource,
    CancellationToken,
    CancellationTokenRegistration};
