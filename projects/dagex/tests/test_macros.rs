use std::collections::HashSet;

use dagex::{const_parse_newick, core::Node};


#[test]
fn test_parser_1() {
    let network = const_parse_newick!(";");
    assert!(network.taxa().is_empty());
    let graph = network.graph();
    assert_eq!(graph.number_of_nodes(), 1);
}

#[test]
fn test_parser_2() {
    let network = const_parse_newick!("(,());");
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
    let network = const_parse_newick!("((A, B),(B, C));");
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
    let network = const_parse_newick!("((A, (D)B#1),(B#1, C));");
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

#[test]
fn test_episode_feasibility_network() {
    let network = const_parse_newick!("((X, (b, ((L1,L2), (L3,L4)))), (d, (c, a)));");
    let graph = network.graph();

    let find_by_taxon = |t: &str| -> Option<Node> {
        let mut iter = network.iter_by_taxon(t);
        let next = iter.next();
        assert!(iter.next().is_none(), "Duplicate taxa");
        next
    };

    let root = network.root();
    assert_eq!(graph.get_successors(root).len(), 2);

    let c_node = find_by_taxon("c").unwrap();
    assert!(network.is_leaf(c_node));
    assert_eq!(graph.get_predecessors(c_node).len(), 1);
    let c_pred = graph.get_predecessors(c_node)[0];
    let a_node = find_by_taxon("a").unwrap();
    assert!(network.is_leaf(a_node));
    assert_eq!(graph.get_predecessors(a_node), [c_pred]);
    assert_eq!(graph.get_predecessors(c_pred).len(), 1);
    let cc_pred = graph.get_predecessors(c_pred)[0];
    let d_node = find_by_taxon("d").unwrap();
    assert!(network.is_leaf(d_node));
    assert_eq!(graph.get_predecessors(d_node), [cc_pred]);

    assert_eq!(graph.get_predecessors(cc_pred), [root]);

    let l1_node = find_by_taxon("L1").unwrap();
    assert!(network.is_leaf(l1_node));
    assert_eq!(graph.get_predecessors(l1_node).len(), 1);
    let l_pred = graph.get_predecessors(l1_node)[0];
    let l2_node = find_by_taxon("L1").unwrap();
    assert!(network.is_leaf(l2_node));
    assert_eq!(graph.get_predecessors(l2_node), [l_pred]);

    let l3_node = find_by_taxon("L3").unwrap();
    assert!(network.is_leaf(l3_node));
    assert_eq!(graph.get_predecessors(l3_node).len(), 1);
    let r_pred = graph.get_predecessors(l3_node)[0];
    let l4_node = find_by_taxon("L4").unwrap();
    assert!(network.is_leaf(l4_node));
    assert_eq!(graph.get_predecessors(l4_node), [r_pred]);

    assert_eq!(graph.get_predecessors(l_pred).len(), 1);
    let z_node = graph.get_predecessors(l_pred)[0];
    assert_eq!(graph.get_predecessors(r_pred), [z_node]);
    assert_eq!(graph.get_predecessors(z_node).len(), 1);
    let v_node = graph.get_predecessors(z_node)[0];
    let b_node = find_by_taxon("b").unwrap();
    assert!(network.is_leaf(b_node));
    assert_eq!(graph.get_predecessors(b_node), [v_node]);

    assert_eq!(graph.get_predecessors(v_node).len(), 1);
    let w_node = graph.get_predecessors(v_node)[0];
    let x_node = find_by_taxon("X").unwrap();
    assert!(network.is_leaf(x_node));
    assert_eq!(graph.get_predecessors(x_node), [w_node]);
    assert_eq!(graph.get_predecessors(w_node), [root]);
}