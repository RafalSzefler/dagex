use std::collections::HashSet;

use dagex::{const_parse_newick, phylo::GenesOverSpecies};
use dagex_algorithms::{
    episode_feasibility::{
        EpisodeFeasabilityAlgorithmFactoryBuilder,
        EpisodeFeasabilityInput},
    traits::{
        Algorithm,
        AlgorithmFactory,
        AlgorithmFactoryBuilder}};


#[test]
fn test_episode_feasibility_algorithm() {
    let genes = const_parse_newick!("((, (b, ((,), (,)))), (d, (c, a)));");
    let genes_id = genes.id();
    let species = const_parse_newick!("((a, c), (b, d));");
    let a_leaf = species.graph().leaves().iter()
        .find(|n| {
            if let Some(taxon) = species.taxa().get(n) {
                return taxon.value().as_str() == "a";
            }
            false
        }).unwrap();
    let episode_candidates = HashSet::from([species.root(), *a_leaf]);
    let genes_over_species = GenesOverSpecies::new_single_gene(genes, species).unwrap();
    let mut factory = EpisodeFeasabilityAlgorithmFactoryBuilder::default().create().unwrap();
    let episode_input = EpisodeFeasabilityInput::new(&genes_over_species, &episode_candidates);
    let algo = factory.create(episode_input).unwrap();
    let result = algo.run().unwrap();
    assert!(result.result().get(&genes_id).unwrap());
}
