use std::collections::HashMap;

use raf_multi_valued_logic::tribool::TriBool;

use crate::traits::Algorithm;

use super::{EpisodeFeasabilityInput, EpisodeFeasabilityOutput, FormulaData};


pub struct EpisodeFeasabilityAlgorithm<'a> {
    input: EpisodeFeasabilityInput<'a>,
}

impl<'a> EpisodeFeasabilityAlgorithm<'a> {
    pub(super) fn new(
        input: EpisodeFeasabilityInput<'a>) -> Self
    {
        Self { input }
    }
}

impl<'a> Algorithm<'a> for EpisodeFeasabilityAlgorithm<'a> {
    type Input<'b> = EpisodeFeasabilityInput<'b>;

    type Output<'b> = EpisodeFeasabilityOutput<'b>;

    type Error = ();

    fn run(self) -> Result<Self::Output<'a>, Self::Error> {
        let episode_candidates = self.input.episode_candidates();
        let genes_over_species = self.input.genes_over_species();
        let species = genes_over_species.species_network();
        let species_root = species.root();
        let genes = genes_over_species.gene_networks();
        let mut result = HashMap::with_capacity(genes.len());
        for gene_network in genes {
            let formula_data = FormulaData::new(gene_network, species, episode_candidates);
            let gene_root = gene_network.root();
            let calc_result = formula_data.delta_down(gene_root, species_root);
            result.insert(gene_network.id(), calc_result == TriBool::TRUE);
        }
        
        Ok(Self::Output::new(result))
    }
}
