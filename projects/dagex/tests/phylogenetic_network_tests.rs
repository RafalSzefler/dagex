use std::collections::{HashMap, HashSet};

use dagex::{
    core::{
        ArrowDTO,
        DirectedGraphDTO,
        DirectedGraphFromResult,
        Node
    },
    phylo::{
        PhylogeneticNetwork,
        PhylogeneticNetworkDTO,
        PhylogeneticNetworkFromResult
    }
};
use immutable_string::ImmutableString;

fn imm(text: &str) -> ImmutableString { ImmutableString::new(text).unwrap() }

fn dg_dto_empty() -> DirectedGraphDTO { DirectedGraphDTO::new(0, Vec::new()) }

fn dg_dto(arrows: &[(i32, i32)]) -> DirectedGraphDTO {
    let mut max = 0;
    let mut target_arrows = Vec::<ArrowDTO>::with_capacity(arrows.len());
    for (source, target) in arrows {
        let s = source.clone();
        let t = target.clone();
        max = core::cmp::max(s, core::cmp::max(t, max));
        target_arrows.push(ArrowDTO::new(s, t));
    }
    DirectedGraphDTO::new(max+1, Vec::from_iter(target_arrows))
}

fn tree_nodes(pn: &PhylogeneticNetwork) -> HashSet<Node> {
    pn.graph().iter_nodes().filter(|n| pn.is_tree_node(*n)).collect()
}

fn reticulation_nodes(pn: &PhylogeneticNetwork) -> HashSet<Node> {
    pn.graph().iter_nodes().filter(|n| pn.is_reticulation_node(*n)).collect()
}

fn cross_nodes(pn: &PhylogeneticNetwork) -> HashSet<Node> {
    pn.graph().iter_nodes().filter(|n| pn.is_cross_node(*n)).collect()
}

fn leaves(pn: &PhylogeneticNetwork) -> HashSet<Node> {
    pn.graph().iter_nodes().filter(|n| pn.is_leaf(*n)).collect()
}

#[test]
fn test_empty() {
    let dto = PhylogeneticNetworkDTO::new(
        dg_dto_empty(),
        HashMap::new());
    
    let result = PhylogeneticNetwork::from_dto(&dto);
    assert!(matches!(result, PhylogeneticNetworkFromResult::GraphError(DirectedGraphFromResult::EmptyGraph)));
}

#[test]
fn test_empty_with_taxa() {
    let dto = PhylogeneticNetworkDTO::new(
        dg_dto_empty(),
        HashMap::from_iter([(1, imm("test"))]));
    
    let result = PhylogeneticNetwork::from_dto(&dto);
    assert!(matches!(result, PhylogeneticNetworkFromResult::GraphError(DirectedGraphFromResult::EmptyGraph)));
}

#[test]
fn test_taxa_not_leaves_1() {
    let dto = PhylogeneticNetworkDTO::new(
        DirectedGraphDTO::new(1, Vec::new()),
        HashMap::from_iter([(1, imm("test"))]));
    
    let result = PhylogeneticNetwork::from_dto(&dto);
    assert!(matches!(result, PhylogeneticNetworkFromResult::TaxaNotLeaves(_)));
}

#[test]
fn test_taxa_not_leaves_2() {
    let dto = PhylogeneticNetworkDTO::new(
        dg_dto(&[(0, 1)]),
        HashMap::from_iter([(0, imm("test2"))]));
    
    let result = PhylogeneticNetwork::from_dto(&dto);
    assert!(matches!(result, PhylogeneticNetworkFromResult::TaxaNotLeaves(_)));
}

#[test]
fn test_ok() {
    let dto = PhylogeneticNetworkDTO::new(
        dg_dto(&[(0, 1), (0, 2)]),
        HashMap::from_iter([(1, imm("a")), (2, imm("xyz"))]));
    
    let result = PhylogeneticNetwork::from_dto(&dto);
    assert!(matches!(result, PhylogeneticNetworkFromResult::Ok(_)), "Invalid result: {result:?}");
    let network = result.unwrap();
    let taxa = network.taxa();
    assert_eq!(taxa.len(), 2);
    assert_eq!(taxa.get(&Node::from(1)).unwrap().as_immutable_string(), &imm("a"));
    assert_eq!(taxa.get(&Node::from(2)).unwrap().as_immutable_string(), &imm("xyz"));

    assert_eq!(leaves(&network), HashSet::from([Node::from(1), Node::from(2)]));
    assert_eq!(tree_nodes(&network), HashSet::from([Node::from(0)]));
    assert_eq!(reticulation_nodes(&network), HashSet::new());
    assert_eq!(cross_nodes(&network), HashSet::new());

    let graph = network.graph();
    let props = graph.basic_properties();
    assert!(props.acyclic);
    assert!(props.connected);
    assert!(props.rooted);
    assert!(props.tree);
    let root = graph.root().unwrap();
    assert_eq!(root.as_i32(), 0);
    let network_root = network.root();
    assert_eq!(network_root, root);

    assert_eq!(graph.number_of_nodes(), 3);
    let node0 = Node::from(0);
    let node1 = Node::from(1);
    let node2 = Node::from(2);

    assert_eq!(root, node0);

    let mut root_successors = Vec::from(graph.get_successors(node0));
    root_successors.sort_by_key(|n| n.as_i32());

    assert_eq!(root_successors, &[node1, node2]);
    assert_eq!(graph.get_predecessors(node0).len(), 0);

    assert_eq!(graph.get_successors(node1).len(), 0);
    assert_eq!(graph.get_predecessors(node1), &[node0]);
    assert_eq!(graph.get_successors(node2).len(), 0);
    assert_eq!(graph.get_predecessors(node2), &[node0]);
}

#[test]
fn test_tree() {
    let dto = PhylogeneticNetworkDTO::new(
        dg_dto(&[(0, 1), (0, 2)]),
        HashMap::new());
    
    let result = PhylogeneticNetwork::from_dto(&dto);
    assert!(matches!(result, PhylogeneticNetworkFromResult::Ok(_)), "Invalid result: {result:?}");
    let network = result.unwrap();

    assert!(network.graph().basic_properties().tree);
}

#[test]
fn test_tree_child() {
    let dto = PhylogeneticNetworkDTO::new(
        dg_dto(&[(0, 1), (0, 2), (1, 3), (1, 4), (2, 4), (2, 5), (4, 6)]),
        HashMap::new());
    
    let result = PhylogeneticNetwork::from_dto(&dto);
    assert!(matches!(result, PhylogeneticNetworkFromResult::Ok(_)), "Invalid result: {result:?}");
    let network = result.unwrap();

    assert_eq!(leaves(&network), HashSet::from([Node::from(3), Node::from(5), Node::from(6)]));
    assert_eq!(tree_nodes(&network), HashSet::from([Node::from(0), Node::from(2), Node::from(1)]));
    assert_eq!(reticulation_nodes(&network), HashSet::from([Node::from(4)]));
    assert_eq!(cross_nodes(&network), HashSet::new());

    assert!(!network.graph().basic_properties().tree);
}


#[test]
fn test_invalid_leaf() {
    let dto = PhylogeneticNetworkDTO::new(
        dg_dto(&[(0, 1), (0, 2), (1, 3), (1, 4), (2, 4), (2, 5)]),
        HashMap::new());
    
    let result = PhylogeneticNetwork::from_dto(&dto);
    assert!(matches!(result, PhylogeneticNetworkFromResult::LeavesNotOfInDegreeOne(_)), "Invalid result: {result:?}");
}
