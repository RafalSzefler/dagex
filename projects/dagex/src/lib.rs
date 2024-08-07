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
extern crate dagex_impl;

#[doc(hidden)]
extern crate dagex_macros;

pub use dagex_impl::*;
pub use dagex_macros::*;
