use std::collections::HashMap;

use dagex::{core::{ArrowDTO, DirectedGraphDTO}, phylo::{GenesOverSpecies, GenesOverSpeciesFromResult, PhylogeneticNetwork, PhylogeneticNetworkDTO}};
use immutable_string::ImmutableString;


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
    let network_dto = PhylogeneticNetworkDTO::new(
        graph_dto,
        mapped_taxa);
    PhylogeneticNetwork::from_dto(&network_dto).unwrap()
}

#[test]
fn test_valid_taxa_1() {
    let genes_network = build_network(
        &[(0, 1)],
        &[(1, "Test")]);
    let genes_network_id = genes_network.get_id();
    let species_network = build_network(
        &[(0, 1), (0, 2)],
        &[(1, "Test"), (2, "baz")]);
    let species_network_id = species_network.get_id();
    let genes_over_species 
        = GenesOverSpecies::from_single_network(genes_network, species_network)
            .unwrap();
    
    let genes = genes_over_species.get_gene_networks();
    assert_eq!(genes.len(), 1);
    assert_eq!(genes_over_species.get_gene_network_by_id(genes[0].get_id()).unwrap().get_id(), genes_network_id);
    assert_eq!(genes_over_species.get_species_network().get_id(), species_network_id);
}


#[test]
fn test_valid_taxa_2() {
    let genes_network = build_network(
        &[(0, 1)],
        &[(1, "Test")]);
    let genes_network_id = genes_network.get_id();
    let species_network = build_network(
        &[(0, 1), (0, 2)],
        &[(1, "Test")]);
    let species_network_id = species_network.get_id();
    let genes_over_species 
        = GenesOverSpecies::from_single_network(genes_network, species_network)
            .unwrap();
    
    let genes = genes_over_species.get_gene_networks();
    assert_eq!(genes.len(), 1);
    assert_eq!(genes_over_species.get_gene_network_by_id(genes[0].get_id()).unwrap().get_id(), genes_network_id);
    assert_eq!(genes_over_species.get_species_network().get_id(), species_network_id);
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
    assert!(matches!(genes_over_species, GenesOverSpeciesFromResult::IncorrectTaxa(_, _)));
}

#[test]
fn test_taxa_duplicates() {
    let genes = build_network(
        &[(0, 1)],
        &[(1, "Baz")]);
    let species = build_network(
        &[(0, 1), (0, 2), (2, 3)],
        &[(1, "Baz"), (3, "Baz")]);
    let genes_over_species = GenesOverSpecies::from_single_network(genes, species);
    assert!(matches!(genes_over_species, GenesOverSpeciesFromResult::SpeciesContainsTaxaDuplicates(_, _)));
}