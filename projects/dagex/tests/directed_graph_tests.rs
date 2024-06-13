use dagex::core::{ArrowDTO, DirectedGraph, DirectedGraphDTO, DirectedGraphFromResult, Node};
use rstest::rstest;

#[test]
fn test_empty() {
    let dto = DirectedGraphDTO::new(0, Vec::new());
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::EmptyGraph), "Invalid result: {result:?}");
}

#[test]
fn test_out_of_range() {
    let over_max = DirectedGraph::get_max_size() + 1;
    let dto = DirectedGraphDTO::new(over_max, Vec::new());
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::TooBigGraph), "Invalid result: {result:?}");
}

#[rstest]
fn test_trivial(#[values(2, 3, 4, 5, 6, 7, 8, 9, 10)] no: i32) {
    let dto = DirectedGraphDTO::new(no, Vec::new());
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::Ok(_)), "Invalid result: {result:?}");
    let graph = result.unwrap();
    assert_eq!(graph.get_number_of_nodes(), no);
    let props = graph.get_basic_properties();
    assert!(props.acyclic);
    assert!(!props.connected);
    assert!(!props.rooted);
    assert!(props.binary);

    let mut node_count = 0;
    for node in graph.iter_nodes() {
        node_count += 1;
        assert_eq!(graph.get_successors(node).len(), 0);
        assert_eq!(graph.get_predecessors(node).len(), 0);
    }

    assert_eq!(node_count, no);
}

fn build_dto(arrows: &[(i32, i32)]) -> DirectedGraphDTO {
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

#[test]
fn test_multi_arrows() {
    let dto = build_dto(&[(0, 1), (1, 0), (0, 1)]);
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::MultipleParallelArrows(_)), "Invalid result: {result:?}");
}

#[test]
fn test_arrows_out_of_range_1() {
    let dto = build_dto(&[(-1, 5)]);
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::ArrowOutsideOfNodesRange(_)), "Invalid result: {result:?}");
}

#[test]
fn test_arrows_out_of_range_2() {
    let dto = DirectedGraphDTO::new(1, Vec::from(&[ArrowDTO::new(0, 5)]));
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::ArrowOutsideOfNodesRange(_)), "Invalid result: {result:?}");
}

#[test]
fn test_cycle() {
    let dto = build_dto(&[(0, 1), (1, 0)]);
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::Ok(_)), "Invalid result: {result:?}");
    let graph = result.unwrap();
    assert_eq!(graph.get_number_of_nodes(), 2);
    let props = graph.get_basic_properties();
    assert!(!props.acyclic);
    assert!(props.connected);
    assert!(!props.rooted);
    assert!(props.binary);
    for node in graph.iter_nodes() {
        assert_eq!(graph.get_successors(node).len(), 1);
        assert_eq!(graph.get_predecessors(node).len(), 1);
    }
}

#[test]
fn test_bigger_cycle() {
    let dto = build_dto(&[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)]);
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::Ok(_)), "Invalid result: {result:?}");
    let graph = result.unwrap();
    assert_eq!(graph.get_number_of_nodes(), 6);
    let props = graph.get_basic_properties();
    assert!(!props.acyclic);
    assert!(props.connected);
    assert!(!props.rooted);
    assert!(props.binary);
    for node in graph.iter_nodes() {
        assert_eq!(graph.get_successors(node).len(), 1);
        assert_eq!(graph.get_predecessors(node).len(), 1);
    }
}

#[test]
fn test_rooted_cycle() {
    let dto = build_dto(&[(0, 1), (1, 0), (2, 0)]);
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::Ok(_)), "Invalid result: {result:?}");
    let graph = result.unwrap();
    assert_eq!(graph.get_number_of_nodes(), 3);
    let props = graph.get_basic_properties();
    assert!(!props.acyclic);
    assert!(props.connected);
    assert!(props.rooted);
    assert!(props.binary);
}


#[test]
fn test_disconnected_cycle() {
    let dto = build_dto(&[(0, 1), (1, 0), (2, 3), (3, 2)]);
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::Ok(_)), "Invalid result: {result:?}");
    let graph = result.unwrap();
    assert_eq!(graph.get_number_of_nodes(), 4);
    let props = graph.get_basic_properties();
    assert!(!props.acyclic);
    assert!(!props.connected);
    assert!(!props.rooted);
    assert!(props.binary);
}

#[test]
fn test_binary() {
    let dto = build_dto(&[(0, 1), (1, 2), (1, 3), (2, 4)]);
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::Ok(_)), "Invalid result: {result:?}");
    let graph = result.unwrap();
    assert_eq!(graph.get_number_of_nodes(), 5);
    let props = graph.get_basic_properties();
    assert!(props.acyclic);
    assert!(props.connected);
    assert!(props.rooted);
    assert!(props.binary);
    assert!(graph.get_root().is_some_and(|val| val.as_i32() == 0));
    let mut leaves = Vec::from_iter(graph.get_leaves().into_iter().map(|n| *n));
    leaves.sort_by_key(Node::as_i32);
    assert_eq!(leaves.len(), 2);
    assert_eq!(leaves[0].as_i32(), 3);
    assert_eq!(leaves[1].as_i32(), 4);
}


#[test]
fn test_non_binary() {
    let dto = build_dto(&[(0, 1), (1, 2), (1, 3), (2, 4), (1, 5)]);
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::Ok(_)), "Invalid result: {result:?}");
    let graph = result.unwrap();
    assert_eq!(graph.get_number_of_nodes(), 6);
    let props = graph.get_basic_properties();
    assert!(props.acyclic);
    assert!(props.connected);
    assert!(props.rooted);
    assert!(!props.binary);
    assert!(graph.get_root().is_some_and(|val| val.as_i32() == 0));
    let mut leaves = Vec::from_iter(graph.get_leaves().into_iter().map(|n| *n));
    leaves.sort_by_key(Node::as_i32);
    assert_eq!(leaves.len(), 3);
    assert_eq!(leaves[0].as_i32(), 3);
    assert_eq!(leaves[1].as_i32(), 4);
    assert_eq!(leaves[2].as_i32(), 5);
}

#[test]
fn test_with_reticulation() {
    let dto = build_dto(&[(0, 1), (1, 2), (1, 3), (2, 4), (3, 5), (2, 5)]);
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::Ok(_)), "Invalid result: {result:?}");
    let graph = result.unwrap();
    assert_eq!(graph.get_number_of_nodes(), 6);
    let props = graph.get_basic_properties();
    assert!(props.acyclic);
    assert!(props.connected);
    assert!(props.rooted);
    assert!(props.binary);
    assert!(graph.get_root().is_some_and(|val| val.as_i32() == 0));
    let mut leaves = Vec::from_iter(graph.get_leaves().into_iter().map(|n| *n));
    leaves.sort_by_key(Node::as_i32);
    assert_eq!(leaves.len(), 2);
    assert_eq!(leaves[0].as_i32(), 4);
    assert_eq!(leaves[1].as_i32(), 5);
}

#[test]
fn test_equality_1() {
    let dto = build_dto(&[(0, 1), (1, 2), (1, 3), (2, 4), (3, 5), (2, 5)]);
    let result = DirectedGraph::from_dto(&dto);
    assert!(matches!(result, DirectedGraphFromResult::Ok(_)), "Invalid result: {result:?}");
    let graph = result.unwrap();
    assert_eq!(graph, graph);
}

#[test]
fn test_equality_2() {
    let dto1 = build_dto(&[(0, 1), (1, 2), (1, 3), (2, 4), (3, 5), (2, 5)]);
    let result1 = DirectedGraph::from_dto(&dto1);
    let graph1 = result1.unwrap();

    let dto2 = build_dto(&[(0, 1), (1, 2), (1, 3), (2, 4), (3, 5), (2, 5)]);
    let result2 = DirectedGraph::from_dto(&dto2);
    let graph2 = result2.unwrap();
    assert_eq!(graph1, graph2);
}

#[test]
fn test_unequal() {
    let dto1 = build_dto(&[(0, 1), (1, 2), (1, 3), (2, 4), (3, 5)]);
    let result1 = DirectedGraph::from_dto(&dto1);
    let graph1 = result1.unwrap();

    let dto2 = build_dto(&[(0, 1), (1, 2), (1, 3), (2, 4), (3, 5), (2, 5)]);
    let result2 = DirectedGraph::from_dto(&dto2);
    let graph2 = result2.unwrap();
    assert_ne!(graph1, graph2);
}
