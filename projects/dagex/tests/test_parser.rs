use std::collections::HashSet;

use dagex::phylo::parse_newick_from_str;


#[test]
fn test_parser_1() {
    let network = parse_newick_from_str(";").unwrap().network;
    assert!(network.taxa().is_empty());
    let graph = network.graph();
    assert_eq!(graph.number_of_nodes(), 1);
}

#[test]
fn test_parser_2() {
    let network = parse_newick_from_str("(,());").unwrap().network;
    assert!(network.taxa().is_empty());
    let graph = network.graph();
    assert_eq!(graph.number_of_nodes(), 4);
    let root = network.root();
    let first_children = graph.get_successors(root);
    assert_eq!(first_children.len(), 2);
    assert_eq!(graph.get_successors(first_children[0]).len(), 0);
    assert_eq!(graph.get_successors(first_children[1]).len(), 1);
    assert_eq!(graph.leaves().len(), 2);
}

#[test]
fn test_parser_3() {
    let network = parse_newick_from_str("((A, B),(B, C));").unwrap().network;
    assert!(!network.taxa().is_empty());
    let graph = network.graph();
    assert_eq!(graph.number_of_nodes(), 7);
    let root = network.root();
    let first_children = graph.get_successors(root);
    assert_eq!(first_children.len(), 2);
    assert_eq!(graph.get_successors(first_children[0]).len(), 2);
    assert_eq!(graph.get_successors(first_children[1]).len(), 2);
    let leaves = graph.leaves();
    assert_eq!(leaves.len(), 4);
    let taxa = network.taxa();
    let leaves_taxas: HashSet<&str> = leaves.iter()
        .map(|x| taxa.get(x).unwrap().value().as_str())
        .collect();
    let expected_taxa = HashSet::from(["A", "B", "C"]);
    assert_eq!(leaves_taxas, expected_taxa);
    let reticulations = graph
        .iter_nodes()
        .filter(|n| network.is_reticulation_node(*n))
        .count();
    assert_eq!(reticulations, 0);
}

#[test]
fn test_parser_4() {
    let network = parse_newick_from_str("((A, (D)B#1),(B#1, C));").unwrap().network;
    assert!(!network.taxa().is_empty());
    let graph = network.graph();
    assert_eq!(graph.number_of_nodes(), 7);
    let root = network.root();
    let first_children = graph.get_successors(root);
    assert_eq!(first_children.len(), 2);
    assert_eq!(graph.get_successors(first_children[0]).len(), 2);
    assert_eq!(graph.get_successors(first_children[1]).len(), 2);
    let leaves = graph.leaves();
    assert_eq!(leaves.len(), 3);
    let taxa = network.taxa();
    let leaves_taxas: HashSet<&str> = leaves.iter()
        .map(|x| taxa.get(x).unwrap().value().as_str())
        .collect();
    let expected_taxa = HashSet::from(["A", "C", "D"]);
    assert_eq!(leaves_taxas, expected_taxa);
    let reticulations = graph
        .iter_nodes()
        .filter(|n| network.is_reticulation_node(*n))
        .count();
    assert_eq!(reticulations, 1);
}
