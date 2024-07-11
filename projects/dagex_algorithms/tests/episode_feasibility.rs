// use std::collections::HashMap;

// use dagex::{core::{ArrowDTO, DirectedGraphDTO}, phylo::{PhylogeneticNetwork, PhylogeneticNetworkDTO}};
// use immutable_string::ImmutableString;


// fn build_phylo(arr: &[(i32, i32)], taxa: &[(i32, &str)]) -> PhylogeneticNetwork {
//     let mut number_of_nodes = -1;
//     let mut arrows = Vec::with_capacity(arr.len());
//     for (src, trg) in arr {
//         let isrc = *src;
//         let itrg = *trg;
//         number_of_nodes = core::cmp::max(
//             number_of_nodes,
//             core::cmp::max(isrc, itrg));
//         arrows.push(ArrowDTO::new(isrc, itrg));
//     }
//     let dto = DirectedGraphDTO::new(number_of_nodes + 1, arrows);
    
//     let mut taxa_map = HashMap::with_capacity(taxa.len());
//     for (node, taxon) in taxa {
//         let imm = ImmutableString::new(taxon).unwrap();
//         taxa_map.insert(*node, imm);
//     }

//     let phylo_dto = PhylogeneticNetworkDTO::new(dto, taxa_map);
//     PhylogeneticNetwork::from_dto(&phylo_dto).unwrap()
// }

// #[test]
// fn test_episode_feasibility_algorithm() {
//     let genes = build_phylo(
//         &[(0, 1),
//         (1, 2),    (1, 3),
//         (2, 4), (2, 5),
//         (5, 6), (5, 7),
//         ],
//         taxa)
//     let graph = build_graph(arrows);
//     let mut factory = DepthAlgorithmFactoryBuilder::default().create().unwrap();
//     let algo = factory.create(&graph).unwrap();
//     let result = algo.run().unwrap();
//     assert_eq!(result.max_depth(), expected);
// }
