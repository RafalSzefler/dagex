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
    assert_eq!(genes.graph().leaves().len(), 9);
    assert_eq!(genes.taxa().len(), 4);
    let genes_id = genes.id();
    let species = const_parse_newick!("((a, c)ac, (b, d)bd)acbd;");
    assert_eq!(species.graph().leaves().len(), 4);
    assert_eq!(species.taxa().len(), 4);
    let a_leaf = species.iter_by_taxon("a").next().unwrap();
    let episode_candidates = HashSet::from([species.root(), a_leaf]);
    let genes_over_species = GenesOverSpecies::new_single_gene(genes, species).unwrap();
    let mut factory = EpisodeFeasabilityAlgorithmFactoryBuilder::default().create().unwrap();
    let episode_input = EpisodeFeasabilityInput::new(&genes_over_species, &episode_candidates);
    let algo = factory.create(episode_input).unwrap();
    let result = algo.run().unwrap();
    let is_feasible = result.result().get(&genes_id).unwrap();
    assert!(is_feasible);
}
