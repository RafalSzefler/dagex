#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]

mod types;
mod construction_error;
mod cache;
mod buffer;
mod string_buffer;
mod fnv1_hash;
mod layout_helpers;
mod immutable_string;
mod weak_immutable_string;

pub use construction_error::ConstructionError;
pub use immutable_string::ImmutableString;
