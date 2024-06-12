#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
)]
mod taxon;
mod phylogenetic_network_dto;
mod phylogenetic_network;
mod genes_over_species;

pub use taxon::Taxon;
pub use phylogenetic_network_dto::PhylogeneticNetworkDTO;
pub use phylogenetic_network::{PhylogeneticNetwork, PhyloConstructionResult};
pub use genes_over_species::{GenesOverSpecies};