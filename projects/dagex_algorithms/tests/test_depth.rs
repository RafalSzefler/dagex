use dagex::core::{ArrowDTO, DirectedGraph, DirectedGraphDTO};
use dagex_algorithms::{depth::DepthAlgorithmFactoryBuilder, traits::{Algorithm, AlgorithmFactory, AlgorithmFactoryBuilder}};
use rstest::rstest;

fn build_graph(arr: &[(i32, i32)]) -> DirectedGraph {
    let mut number_of_nodes = -1;
    let mut arrows = Vec::with_capacity(arr.len());
    for (src, trg) in arr {
        let isrc = *src;
        let itrg = *trg;
        number_of_nodes = core::cmp::max(
            number_of_nodes,
            core::cmp::max(isrc, itrg));
        arrows.push(ArrowDTO::new(isrc, itrg));
    }
    let dto = DirectedGraphDTO::new(number_of_nodes + 1, arrows);
    DirectedGraph::from_dto(&dto).unwrap()
}

#[rstest]
#[case(&[(0, 1)], 1)]
#[case(&[(0, 1), (0, 2)], 1)]
#[case(&[(0, 1), (0, 2), (1, 3)], 2)]
#[case(&[(0, 1), (1, 2), (2, 3)], 3)]
#[case(&[(0, 1), (1, 2), (2, 3), (0, 4)], 3)]
fn test_depth_algorithm(#[case] arrows: &[(i32, i32)], #[case] expected: i32) {
    let graph = build_graph(arrows);
    let mut factory = DepthAlgorithmFactoryBuilder::default().create().unwrap();
    let algo = factory.create(&graph).unwrap();
    let result = algo.run().unwrap();
    assert_eq!(result.max_depth(), expected);
}
