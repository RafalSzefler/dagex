#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod converter;

#[allow(unused_imports)]
use dagex_impl::phylo::{parse_newick_from_str, PhylogeneticNetwork};

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

/// Constructs [`PhylogeneticNetwork`] from Newick string at compile time.
/// 
/// # Panics
/// Whenever can't construct the network, according to [`parse_newick_from_str`]
/// errors.
#[proc_macro]
pub fn const_parse_newick(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let text = input.value();
    let network = parse_newick_from_str(&text)
        .unwrap()
        .network;
    converter::convert(&network).into()
}
