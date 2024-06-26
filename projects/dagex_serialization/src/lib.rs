#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod traits;
mod traits_serializer;
mod traits_deserializer;

pub use traits::{TypeInfo, WithTypeInfo};
pub use traits_serializer::{Serializer, WriteResult, WriteError};
pub use traits_deserializer::{Deserializer, ReadResult, ReadError};

mod binary_serializer;
mod binary_deserializer;

pub mod binary {
    pub use super::binary_serializer::BinarySerializer;
    pub use super::binary_deserializer::BinaryDeserializer;
}

