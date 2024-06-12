use std::collections::HashSet;

use crate::{PhylogeneticNetwork, Taxon};

pub struct GenesOverSpecies {
    gene_networks: Vec<PhylogeneticNetwork>,
    species_network: PhylogeneticNetwork,
}

pub enum GeneseOverSpeciesFromResult {
    /// Correct `GenesOverSpecies` object.
    Ok(GenesOverSpecies),

    /// Incorrect taxa on some gene network, i.e. not a subset of species
    /// network's taxa. Attached values are returned parameters.
    IncorrectTaxa(Vec<PhylogeneticNetwork>, PhylogeneticNetwork),
}

impl GeneseOverSpeciesFromResult {
    /// Unwraps `FromNetworksResult::Ok` value.
    /// 
    /// # Panics
    /// Only and always when `self` is not `FromNetworksResult::Ok`.
    #[inline(always)]
    pub fn unwrap(self) -> GenesOverSpecies {
        match self {
            GeneseOverSpeciesFromResult::Ok(genes_over_species)
                => genes_over_species,
            GeneseOverSpeciesFromResult::IncorrectTaxa(_, _)
                => panic!("FromNetworksResult not Ok."),
        }
    }
}

impl GenesOverSpecies {
    /// Creates an unchecked `GenesOverSpecies`.
    /// 
    /// # Safety
    /// It is up to caller to ensure that all properties and invariants are
    /// satisfied and consistent. The following invariants have to
    /// be satisfied:
    /// * `gene_network` taxa has to be a subset of `species_network` taxa
    ///   for each `gene_network` in `gene_networks`.
    pub unsafe fn new_unchecked(
        gene_networks: Vec<PhylogeneticNetwork>,
        species_network: PhylogeneticNetwork) -> Self
    {
        Self { gene_networks, species_network }
    }

    pub fn from_networks(
        gene_networks: Vec<PhylogeneticNetwork>,
        species_network: PhylogeneticNetwork) -> GeneseOverSpeciesFromResult
    {
        let species_taxa: HashSet<Taxon>
            = species_network.get_taxa().values().cloned().collect();
        
        for gene_network in &gene_networks {
            if !has_valid_taxa(gene_network, &species_taxa) {
                return GeneseOverSpeciesFromResult::IncorrectTaxa(gene_networks, species_network);
            }
        }

        let result = unsafe {
            Self::new_unchecked(gene_networks, species_network)
        };

        return GeneseOverSpeciesFromResult::Ok(result);
    }

    pub fn from_single_network(
        gene_network: PhylogeneticNetwork,
        species_network: PhylogeneticNetwork) -> GeneseOverSpeciesFromResult
    {
        Self::from_networks(vec![gene_network], species_network)
    }

    #[inline(always)]
    pub fn get_gene_networks(&self) -> &[PhylogeneticNetwork] {
        &self.gene_networks
    }

    #[inline(always)]
    pub fn get_species_network(&self) -> &PhylogeneticNetwork {
        &self.species_network
    }
}


fn has_valid_taxa(
    gene_network: &PhylogeneticNetwork,
    species_taxa: &HashSet<Taxon>) -> bool
{
    let gene_taxa: HashSet<Taxon>
        = gene_network.get_taxa().values().cloned().collect();
    gene_taxa.is_subset(species_taxa)
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use dagex_core::{ArrowDTO, DirectedGraphDTO};
    use immutable_string::ImmutableString;

    use crate::PhylogeneticNetworkDTO;

    use super::*;
    
    fn build_network(arrows: &[(i32, i32)], taxa: &[(i32, &'static str)]) -> PhylogeneticNetwork {
        let mut max = 0;
        let mut target_arrows = Vec::<ArrowDTO>::with_capacity(arrows.len());
        for (source, target) in arrows {
            let s = source.clone();
            let t = target.clone();
            max = core::cmp::max(s, core::cmp::max(t, max));
            target_arrows.push(ArrowDTO::new(s, t));
        }
        let graph_dto = DirectedGraphDTO::new(max+1, Vec::from_iter(target_arrows));
        let mapped_taxa: HashMap<i32, ImmutableString>
            = taxa.iter()
                .map(|kvp| (kvp.0, ImmutableString::get(kvp.1).unwrap()))
                .collect();
        let network_dto = PhylogeneticNetworkDTO::new(graph_dto, mapped_taxa);
        PhylogeneticNetwork::from_dto(&network_dto).unwrap()
    }

    #[test]
    fn test_valid_taxa_1() {
        let genes = build_network(
            &[(0, 1)],
            &[(1, "Test")]);
        let species = build_network(
            &[(0, 1), (0, 2)],
            &[(1, "Test"), (2, "baz")]);
        let genes_over_species = GenesOverSpecies::from_single_network(genes, species);
        assert!(matches!(genes_over_species, GeneseOverSpeciesFromResult::Ok(_)));
    }

    
    #[test]
    fn test_valid_taxa_2() {
        let genes = build_network(
            &[(0, 1)],
            &[(1, "Test")]);
        let species = build_network(
            &[(0, 1), (0, 2), (2, 3)],
            &[(1, "Test")]);
        let genes_over_species = GenesOverSpecies::from_single_network(genes, species);
        assert!(matches!(genes_over_species, GeneseOverSpeciesFromResult::Ok(_)));
    }

    #[test]
    fn test_invalid_taxa() {
        let genes = build_network(
            &[(0, 1)],
            &[(1, "Test")]);
        let species = build_network(
            &[(0, 1), (0, 2), (2, 3)],
            &[(1, "Baz")]);
        let genes_over_species = GenesOverSpecies::from_single_network(genes, species);
        assert!(matches!(genes_over_species, GeneseOverSpeciesFromResult::IncorrectTaxa(_, _)));
    }
}
