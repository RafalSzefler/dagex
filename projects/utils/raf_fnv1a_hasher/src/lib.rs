#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod calculations;
pub use calculations::update_fnv1a_32;

pub(crate) mod init;
mod fnv1a_32_hasher;

pub use fnv1a_32_hasher::FNV1a32Hasher;
