#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod serializer_traits;
mod deserializer_traits;

pub use serializer_traits::*;
pub use deserializer_traits::*;

pub mod binary;
