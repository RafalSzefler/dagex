use std::{collections::HashSet, hash::Hasher};

use dagex::{core::Node, phylo::GenesOverSpecies};

#[derive(PartialEq, Eq)]
pub struct EpisodeFeasabilityInput<'a> {
    genes_over_species: &'a GenesOverSpecies,
    episode_candidates: &'a HashSet<Node>,
}

impl<'a> EpisodeFeasabilityInput<'a> {
    pub fn new(
        genes_over_species: &'a GenesOverSpecies,
        episode_candidates: &'a HashSet<Node>) -> Self
    {
        Self { genes_over_species, episode_candidates }
    }
    
    pub fn genes_over_species(&self) -> &GenesOverSpecies {
        &self.genes_over_species
    }

    pub fn episode_candidates(&self) -> &HashSet<Node> {
        &self.episode_candidates
    }
}

impl<'a> core::hash::Hash for EpisodeFeasabilityInput<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.genes_over_species.hash(state);

        let mut total_hash = self.episode_candidates.len() as u64;
        for node in self.episode_candidates {
            let mut fnv1 = fnv1a_hasher::FNV1a32Hasher::new();
            node.hash(&mut fnv1);
            total_hash ^= fnv1.finish();
        }
        state.write_u64(total_hash);
    }
}
