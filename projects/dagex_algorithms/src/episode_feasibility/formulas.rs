use std::collections::HashSet;

use dagex::{core::Node, phylo::PhylogeneticNetwork};
use raf_multi_valued_logic::tribool::TriBool;

#[derive(Debug, Clone)]
pub struct FormulaData<'a> {
    genes: &'a PhylogeneticNetwork,
    species: &'a PhylogeneticNetwork,
    episode_candidates: &'a HashSet<Node>,
}

impl<'a> FormulaData<'a> {
    pub fn new(
        genes: &'a PhylogeneticNetwork,
        species: &'a PhylogeneticNetwork,
        episode_candidates: &'a HashSet<Node>,
    ) -> Self {
        Self { genes, species, episode_candidates }
    }

    pub fn delta(&self, gene_node: Node, species_node: Node) -> TriBool {
        if self.genes.is_tree_node(gene_node) {
            let result = self.delta_star(gene_node, species_node);
            if self.episode_candidates.contains(&species_node) {
                return result;
            }
            return result.and(TriBool::UNKNOWN);
        }

        TriBool::FALSE
    }

    pub fn delta_down(&self, gene_node: Node, species_node: Node) -> TriBool {
        let mut epsilon_result = self.epsilon(gene_node, species_node);
        if self.species.is_leaf(species_node) {
            return epsilon_result;
        }

        let is_candidate = self.episode_candidates.contains(&species_node);

        let apply_is_possible = if is_candidate {
            |tri: TriBool| { tri.is_possible() }
        } else {
            |tri: TriBool| { tri }
        };

        for successor in self.species.graph().get_successors(species_node) {
            let successor_result = self.delta_down(gene_node, *successor);
            let modified = apply_is_possible(successor_result);
            epsilon_result = epsilon_result.or(modified);
        }

        epsilon_result
    }

    pub fn sigma(&self, gene_node: Node, species_node: Node) -> TriBool {
        let genes = &self.genes;
        let species = &self.species;
        if !genes.is_leaf(gene_node) && !species.is_leaf(species_node) {
            let gsucc = genes.graph().get_successors(gene_node);
            let ssucc = species.graph().get_successors(species_node);
            assert_eq!(gsucc.len(), 2, "Internal gene node has to have two successors.");
            assert_eq!(ssucc.len(), 2, "Internal species node has to have two successors.");
            let left_g = gsucc[0];
            let right_g = gsucc[1];
            let left_s = ssucc[0];
            let right_s = ssucc[1];
            let left_delta = self.delta_down(left_g, left_s)
                .and(self.delta_down(right_g, right_s));
            let right_delta = self.delta_down(left_g, right_s)
                .and(self.delta_down(right_g, left_s));
            return left_delta.or(right_delta).is_certain();
        }

        if genes.is_leaf(gene_node) && species.is_leaf(species_node) {
            let opt_genes_taxon = genes.taxa().get(&gene_node);
            if let Some(genes_taxon) = opt_genes_taxon {
                let species_taxon = species.taxa().get(&species_node).unwrap();
                if species_taxon == genes_taxon {
                    return TriBool::TRUE;
                }
            } else {
                return TriBool::TRUE;
            }
        }

        TriBool::FALSE
    }

    fn epsilon(&self, gene_node: Node, species_node: Node) -> TriBool {
        let sigma_result = self.sigma(gene_node, species_node);
        if sigma_result == TriBool::TRUE {
            return TriBool::TRUE;
        }
        let delta_result = self.delta(gene_node, species_node);
        sigma_result.or(delta_result)
    }

    fn delta_star(&self, gene_node: Node, species_node: Node) -> TriBool {
        let gene_successors = self.genes.graph().get_successors(gene_node);
        let gene_successors_len = gene_successors.len();
        assert!(gene_successors_len == 2, "Expected 2 gene successors, got {gene_successors_len}.");
        let left_gene = gene_successors[0];
        let right_gene = gene_successors[1];

        let mut left_result = self.epsilon(left_gene, species_node);
        left_result = left_result.and(self.delta_down(right_gene, species_node));

        if left_result == TriBool::TRUE {
            return TriBool::TRUE;
        }

        let mut right_result = self.epsilon(right_gene, species_node);
        right_result = right_result.and(self.delta_down(left_gene, species_node));

        left_result.or(right_result)
    }
}
