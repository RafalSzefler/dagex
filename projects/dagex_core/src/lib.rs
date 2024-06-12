#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod node;
mod directed_graph_dto;
mod directed_graph;

pub use node::Node;
pub use directed_graph_dto::{ArrowDTO, DirectedGraphDTO};
pub use directed_graph::{DirectedGraph, DirectedGraphConstructionResult};
