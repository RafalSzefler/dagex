#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]

#[doc(hidden)]
pub extern crate raf_immutable_string;

#[doc(hidden)]
pub mod macro_helpers;

mod impl_serde;
mod hashing;
mod global_id;

pub(crate) use global_id::GlobalId;
pub(crate) use hashing::create_u32_hasher;

pub mod core;
pub mod phylo;