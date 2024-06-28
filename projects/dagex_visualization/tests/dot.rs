use dagex::core::{ArrowDTO, DirectedGraph, DirectedGraphDTO};

use dagex_visualization::dot::{traits::DotSerializer, DefaultDotSerializer};

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
#[case(&[(0, 1)], r"digraph dagex_([0-9]+) \{\s*0 -> \{ 1 \};\s*\}\s*")]
#[case(&[(0, 1), (0, 2)], r"digraph dagex_([0-9]+) \{\s*0 -> \{ 1 2 \};\s*\}\s*")]
#[case(&[(0, 1), (0, 2), (1, 3)], r"digraph dagex_([0-9]+) \{\s*0 -> \{ 1 2 \};\s*1 -> \{ 3 \};\s*\}\s*")]
#[case(&[(0, 1), (1, 2), (2, 3)], r"digraph dagex_([0-9]+) \{\s*0 -> \{ 1 \};\s*1 -> \{ 2 \};\s*2 -> \{ 3 \};\s*\}\s*")]
#[case(&[(0, 1), (1, 2), (2, 3), (0, 4)], r"digraph dagex_([0-9]+) \{\s*0 -> \{ 1 4 \};\s*1 -> \{ 2 \};\s**2 -> \{ 3 \};\s*\}\s*")]
fn test_dot_serialization(#[case] arrows: &[(i32, i32)], #[case] expected: &str) {
    use regex::Regex;

    let graph = build_graph(arrows);
    let dto = graph.into_dto();
    let stream = Vec::new();
    let mut serializer = DefaultDotSerializer::from(stream);
    serializer.serialize(&dto).unwrap();
    let stream = serializer.release();
    let text_buf = core::str::from_utf8(stream.as_slice()).unwrap();
    println!("{}", text_buf);
    let re = Regex::new(expected).unwrap();
    assert!(re.is_match(text_buf));
}
