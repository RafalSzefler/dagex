#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]

use proc_macro::TokenStream;

#[proc_macro]
pub fn newick_to_dag(input: TokenStream) -> TokenStream {
    let _ = input;
    panic!("to be implemented");
}
