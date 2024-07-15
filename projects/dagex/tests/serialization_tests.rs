#![cfg(feature="serde")]

use dagex::{core::{ArrowDTO, DirectedGraphDTO}, phylo::PhylogeneticNetworkDTO};
use dagex::ImmutableString;
use rstest::rstest;

#[rstest]
#[case(0, 0, r#"[0,0]"#)]
#[case(-1, 74, r#"[-1,74]"#)]
#[case(16, -3, r#"[16,-3]"#)]
#[case(-124, -87312, r#"[-124,-87312]"#)]
fn test_arrow_dto(
    #[case] source: i32,
    #[case] target: i32,
    #[case] expected: &str)
{
    let arrow = ArrowDTO::new(source, target);
    let result = serde_json::to_string(&arrow).unwrap();
    assert_eq!(result, expected);

    let darrow: ArrowDTO = serde_json::from_str(expected).unwrap();
    assert_eq!(darrow, arrow);
}


#[rstest]
#[case(0, &[(0, 0), (1, -1)], r#"{"number_of_nodes":0,"arrows":[[0,0],[1,-1]]}"#)]
#[case(-5, &[(1, 3)], r#"{"number_of_nodes":-5,"arrows":[[1,3]]}"#)]
#[case(14, &[], r#"{"number_of_nodes":14,"arrows":[]}"#)]
fn test_graph_dto(
    #[case] no: i32,
    #[case] arrows: &[(i32, i32)],
    #[case] expected: &str)
{
    let arrows: Vec<ArrowDTO> = arrows.iter().map(|p| ArrowDTO::new(p.0, p.1)).collect();
    let graph = DirectedGraphDTO::new(no, arrows);
    let result = serde_json::to_string(&graph).unwrap();
    assert_eq!(result, expected);

    let dgraph: DirectedGraphDTO = serde_json::from_str(expected).unwrap();
    assert_eq!(dgraph, graph);
}


#[rstest]
#[case(0, &[(0, 0), (1, -1)], &[(0, "A")], r#"{"number_of_nodes":0,"arrows":[[0,0],[1,-1]],"taxa":[[0,"A"]]}"#)]
#[case(-1, &[(0, 0), (1, -1)], &[], r#"{"number_of_nodes":-1,"arrows":[[0,0],[1,-1]],"taxa":[]}"#)]
#[case(16, &[], &[(15, "")], r#"{"number_of_nodes":16,"arrows":[],"taxa":[[15,""]]}"#)]
#[case(-300, &[], &[], r#"{"number_of_nodes":-300,"arrows":[],"taxa":[]}"#)]
fn test_phylogenetic_dto(
    #[case] no: i32,
    #[case] arrows: &[(i32, i32)],
    #[case] taxa: &[(i32, &str)],
    #[case] expected: &str)
{
    use std::collections::HashMap;

    let arrows: Vec<ArrowDTO> = arrows.iter().map(|p| ArrowDTO::new(p.0, p.1)).collect();
    let graph = DirectedGraphDTO::new(no, arrows);
    let taxa: HashMap<i32, ImmutableString> = taxa.iter().map(|p| (p.0, ImmutableString::new(p.1).unwrap())).collect();
    let pn = PhylogeneticNetworkDTO::new(graph, taxa);
    let result = serde_json::to_string(&pn).unwrap();
    assert_eq!(result, expected);

    let dpn: PhylogeneticNetworkDTO = serde_json::from_str(expected).unwrap();
    assert_eq!(dpn, pn);
}
