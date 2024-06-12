use std::collections::{HashMap, HashSet};

use smallvec::SmallVec;

use crate::{directed_graph_dto::ArrowDTO, DirectedGraphDTO, Node};

type ArrowMap = Vec<SmallVec<[Node; 2]>>;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct DirectedGraphBasicProperties {
    /// The corresponding graph does not contain oriented cycles.
    pub acyclic: bool,

    /// The corresponding graph is connected in unoriented sense.
    pub connected: bool,

    /// The corresponding graph has a single node with in-degree 0, i.e.
    /// without arrows pointing to it.
    pub rooted: bool,
}

/// Represents directed graph. The graph is expected to have a single arrow
/// between any two nodes, i.e. it is not a multigraph. Arrows in opposite
/// directions are allowed.
pub struct DirectedGraph {
    number_of_nodes: i32,
    successors_map: ArrowMap,
    predecessors_map: ArrowMap,
    basic_properties: DirectedGraphBasicProperties,
    root_node: Option<Node>,
    leaves: Vec<Node>,
}

static _EMPTY: &[Node] = &[];


impl DirectedGraph {
    #[inline(always)]
    pub const fn get_max_size() -> i32 {
        1 << 22
    }

    /// Retrieves total numbers of nodes in the graph.
    #[inline(always)]
    pub fn get_number_of_nodes(&self) -> i32 {
        self.number_of_nodes
    }

    #[inline(always)]
    pub fn iter_nodes(&self) -> impl Iterator<Item=Node> {
        (0..self.number_of_nodes).map(Node::new)
    }

    #[inline(always)]
    pub fn get_successors(&self, node: Node) -> &[Node] {
        get_from_arrow_map(node, &self.successors_map)
    }

    #[inline(always)]
    pub fn get_predecessors(&self, node: Node) -> &[Node] {
        get_from_arrow_map(node, &self.predecessors_map)
    }

    #[inline(always)]
    pub fn get_basic_properties(&self) -> &DirectedGraphBasicProperties {
        &self.basic_properties
    }

    /// Returns the single node with in-degree 0 (i.e. without predecessors)
    /// if it exists.
    #[inline(always)]
    pub fn get_root(&self) -> Option<Node> {
        self.root_node
    }

    /// Returns all nodes with out-degree 0 (i.e. without successors).
    #[inline(always)]
    pub fn get_leaves(&self) -> &[Node] {
        &self.leaves
    }

    pub fn into_dto(&self) -> DirectedGraphDTO {
        let max_arrows = core::cmp::max(
            self.successors_map.len(),
            8);
        let mut arrows = Vec::<ArrowDTO>::with_capacity(max_arrows);
        for idx in 0..self.number_of_nodes {
            let node = Node::new(idx);
            for successor in self.get_successors(node) {
                let arrow = ArrowDTO::new(
                    node.get_numeric_id(), 
                    successor.get_numeric_id());
                arrows.push(arrow);
            }
        }
        DirectedGraphDTO::new(self.number_of_nodes, arrows)
    }
}


#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss)]
#[inline(always)]
fn get_from_arrow_map(node: Node, arrow_map: &ArrowMap) -> &[Node] {
    let numeric_id = node.get_numeric_id();
    if numeric_id < 0 || numeric_id > (arrow_map.len() as i32) {
        _EMPTY
    }
    else
    {
        &arrow_map[numeric_id as usize]
    }
}


pub enum DirectedGraphConstructionResult {
    Ok(DirectedGraph),

    /// Passed graph didn't have nodes.
    EmptyGraph,

    /// Graph size exceeded `DirectedGraph::get_max_size()`.
    TooBigGraph,

    /// Graph has multiple arrows between fixed (A, B) nodes. Returns the first
    /// conflicting arrow found.
    MultipleParallelArrows(ArrowDTO),

    /// Graph has arrows outside of range. Either with negative values, or
    /// with value exceeding the number of nodes. Returns the first
    /// conflicting arrow found.
    ArrowOutsideOfNodesRange(ArrowDTO),
}

impl DirectedGraphConstructionResult {
    /// Unwraps `DirectedGraphConstructionResult::Ok` value.
    /// 
    /// # Panics
    /// Only and always when `self` is not `DirectedGraphConstructionResult::Ok`.
    #[inline(always)]
    pub fn unwrap(self) -> DirectedGraph {
        match self {
            DirectedGraphConstructionResult::Ok(graph) => graph,
            _ => panic!("DirectedGraphConstructionResult is not ok."),
        }
    }
}

impl DirectedGraph {
    /// Creates new `DirectedGraph` out of `DirectedGraphDTO`.
    /// 
    /// # Errors
    /// For specific errors read `FromError` docs.
    pub fn from_dto(value: &DirectedGraphDTO)
        -> DirectedGraphConstructionResult
    {
        let number_of_nodes = value.get_number_of_nodes();
        if number_of_nodes <= 0 {
            return DirectedGraphConstructionResult::EmptyGraph;
        }

        if number_of_nodes > Self::get_max_size() {
            return DirectedGraphConstructionResult::TooBigGraph;
        }

        let mut successor_map_duplicates 
            = HashMap::<Node, HashSet<Node>>::new();
        let mut predecessor_map_duplicates 
            = HashMap::<Node, HashSet<Node>>::new();
        let mut properties 
            = DirectedGraphBasicProperties {
                acyclic: false,
                connected: false,
                rooted: false
            };
        let mut root_node = Option::<Node>::None;
        let mut multiple_roots = false;
        let mut leaves = Vec::<Node>::with_capacity(8);

        let arrows = value.get_arrows();
        let mut multi_arrows = HashSet::<ArrowDTO>::with_capacity(arrows.len());

        for arrow in arrows {
            if multi_arrows.contains(arrow) {
                return DirectedGraphConstructionResult::MultipleParallelArrows(arrow.clone());
            }
            multi_arrows.insert(arrow.clone());
            let source = arrow.get_source();
            let target = arrow.get_target();
            if source < 0
                || source >= number_of_nodes
                || target < 0
                || target >= number_of_nodes
            {
                return DirectedGraphConstructionResult::ArrowOutsideOfNodesRange(arrow.clone());
            }
            let source_node = Node::new(source);
            let target_node = Node::new(target);
            insert_node_to_arrow_map(
                source_node,
                target_node,
                &mut successor_map_duplicates);
            insert_node_to_arrow_map(
                target_node,
                source_node,
                &mut predecessor_map_duplicates);
        }

        let successors_map
            = to_arrow_map(number_of_nodes, &successor_map_duplicates);
        let predecessors_map
            = to_arrow_map(number_of_nodes, &predecessor_map_duplicates);

        #[allow(clippy::cast_sign_loss)]
        for idx in 0..number_of_nodes {
            let node = Node::new(idx);
            if predecessors_map[idx as usize].is_empty() {
                if root_node.is_none() {
                    root_node = Some(node);
                }
                else
                {
                    multiple_roots = true;
                }
            }

            if successors_map[idx as usize].is_empty() {
                leaves.push(node);
            }
        }

        if root_node.is_some() && !multiple_roots {
            properties.rooted = true;
        }
        else
        {
            root_node = Option::None;
            properties.rooted = false;
        }

        properties.acyclic = verify_acyclic(number_of_nodes, &successors_map);
        if properties.rooted && properties.acyclic {
            properties.connected = true;
        }
        else
        {
            properties.connected = verify_connected(
                number_of_nodes, 
                &predecessors_map,
                &successors_map);
        }

        let dg = unsafe {
            Self::new_unchecked(
                number_of_nodes,
                successors_map,
                predecessors_map,
                properties,
                root_node,
                leaves)
        };

        DirectedGraphConstructionResult::Ok(dg)
    }

    /// Creates an unchecked `DirectedGraph`.
    /// 
    /// # Safety
    /// It is up to caller to ensure that all properties and invariants are
    /// satisfied and consistent. This also exposes internal structure of
    /// `DirectedGraph`. Use with caution. The following invariants have to
    /// be satisfied:
    /// * `number_of_nodes > 0`.
    /// * `successors_map` and `predecessors_map` are of length equalt
    ///   to `number_of_nodes`.
    /// * each value in `successors_map` and `predecessors_map` contains nodes within
    ///   `(0..number_of_nodes)` range.
    /// * acyclic, rooted and connected pieces of `properties` have to match the
    ///   actual graph structure.
    /// * `root_node` has to point to a single node without a predecessor. It
    ///   has to be `None` if either there is no root, or multiple are present.
    /// * `leaves` have to in `(0..number_of_nodes)` range, have to contain
    ///   nodes without successors, and have to be a complete list of such nodes
    ///   in th graph.
    pub unsafe fn new_unchecked(
            number_of_nodes: i32,
            successors_map: Vec<SmallVec<[Node; 2]>>,
            predecessors_map: Vec<SmallVec<[Node; 2]>>,
            properties: DirectedGraphBasicProperties,
            root_node: Option<Node>,
            leaves: Vec<Node>) -> DirectedGraph
    {
        Self {
            number_of_nodes: number_of_nodes,
            successors_map: successors_map,
            predecessors_map: predecessors_map,
            basic_properties: properties,
            root_node: root_node,
            leaves: leaves,
        }
    }
}


#[allow(clippy::cast_sign_loss)]
fn verify_connected(
    number_of_nodes: i32,
    predecessor_map: &ArrowMap,
    successors_map: &ArrowMap) -> bool
{
    let mut reachable_nodes: HashSet<Node> 
        = (0..number_of_nodes).map(Node::new).collect();
    let first = Node::new(0);

    let mut seen
        = HashSet::<Node>::with_capacity(number_of_nodes as usize);
    verify_connected_remove_all_reachable(
        first,
        &mut reachable_nodes,
        &mut seen,
        predecessor_map,
        successors_map);
    reachable_nodes.is_empty()
}

#[allow(clippy::cast_sign_loss)]
fn verify_connected_remove_all_reachable(
    node: Node,
    reachable_nodes: &mut HashSet<Node>,
    seen: &mut HashSet<Node>,
    predecessor_map: &ArrowMap,
    successors_map: &ArrowMap)
{
    if seen.contains(&node) {
        return;
    }
    seen.insert(node);
    reachable_nodes.remove(&node);
    let idx = node.get_numeric_id() as usize;

    for pred in &predecessor_map[idx] {
        verify_connected_remove_all_reachable(
            *pred,
            reachable_nodes,
            seen,
            predecessor_map,
            successors_map);
    }

    for succ in &successors_map[idx] {
        verify_connected_remove_all_reachable(
            *succ,
            reachable_nodes,
            seen,
            predecessor_map,
            successors_map);
    }
}

fn verify_acyclic(number_of_nodes: i32, successors_map: &ArrowMap) -> bool {
    let mut nodes_stack: Vec<Node> 
        = (0..number_of_nodes).map(Node::new).collect();

    loop {
        if let Some(top) = nodes_stack.pop() {
            let mut seen = HashSet::<Node>::new();
            if verify_acyclic_check_cycle(top, &mut seen, successors_map) {
                return false;
            }
        }
        else
        {
            return true;
        }
    }
}

#[allow(clippy::cast_sign_loss)]
fn verify_acyclic_check_cycle(
    node: Node,
    seen: &mut HashSet<Node>,
    successors_map: &ArrowMap) -> bool
{
    if seen.contains(&node) {
        return true;
    }

    let succs = &successors_map[node.get_numeric_id() as usize];
    if !succs.is_empty() {
        seen.insert(node);
        for successor in succs {
            if verify_acyclic_check_cycle(*successor, seen, successors_map) {
                return true;
            }
        }
        seen.remove(&node);
    }

    return false;
}

#[allow(clippy::cast_sign_loss)]
fn to_arrow_map(number_of_nodes: i32, map: &HashMap<Node, HashSet<Node>>)
    -> ArrowMap
{
    let mut result
        = Vec::<SmallVec<[Node; 2]>>::with_capacity(number_of_nodes as usize);

    for idx in 0..number_of_nodes {
        let node = &Node::new(idx);
        if let Some(set) = map.get(node) {
            let mut vec 
                = SmallVec::<[Node; 2]>::with_capacity(set.len());
            for target in set {
                vec.push(*target);
            }
            result.push(vec);
        }
        else
        {
            result.push(SmallVec::new());
        }
    }

    result
}

fn insert_node_to_arrow_map(
    key: Node, 
    value: Node,
    map: &mut HashMap<Node, HashSet<Node>>)
{
    if let Some(vec) = map.get_mut(&key) {
        vec.insert(value);
    }
    else
    {
        let mut targets
            = HashSet::<Node>::with_capacity(2);
        targets.insert(value);
        map.insert(key, targets);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let dto = DirectedGraphDTO::new(0, Vec::new());
        let result = DirectedGraph::from_dto(&dto);
        assert!(matches!(result, DirectedGraphConstructionResult::EmptyGraph));
    }

    #[test]
    fn test_out_of_range() {
        let over_max = DirectedGraph::get_max_size() + 1;
        let dto = DirectedGraphDTO::new(over_max, Vec::new());
        let result = DirectedGraph::from_dto(&dto);
        assert!(matches!(result, DirectedGraphConstructionResult::TooBigGraph));
    }

    #[test]
    fn test_trivial() {
        for no in 2..10 {
            let dto = DirectedGraphDTO::new(no, Vec::new());
            let result = DirectedGraph::from_dto(&dto);
            let graph = result.unwrap();
            assert_eq!(graph.get_number_of_nodes(), no);
            let props = graph.get_basic_properties();
            assert!(props.acyclic);
            assert!(!props.connected);
            assert!(!props.rooted);

            let mut node_count = 0;
            for node in graph.iter_nodes() {
                node_count += 1;
                assert_eq!(graph.get_successors(node).len(), 0);
                assert_eq!(graph.get_predecessors(node).len(), 0);
            }

            assert_eq!(node_count, no);
        }
    }

    fn build_dto<T: Into<i32> + Clone>(arrows: &[(T, T)]) -> DirectedGraphDTO {
        let mut max = 0;
        let mut target_arrows = Vec::<ArrowDTO>::with_capacity(arrows.len());
        for (source, target) in arrows {
            let s = source.clone().into();
            let t = target.clone().into();
            max = core::cmp::max(s, core::cmp::max(t, max));
            target_arrows.push(ArrowDTO::new(s, t));
        }
        DirectedGraphDTO::new(max+1, Vec::from_iter(target_arrows))
    }

    #[test]
    fn test_multi_arrows() {
        let dto = build_dto(&[(0, 1), (1, 0), (0, 1)]);
        let result = DirectedGraph::from_dto(&dto);
        assert!(matches!(result, DirectedGraphConstructionResult::MultipleParallelArrows(_)));
    }

    #[test]
    fn test_arrows_out_of_range_1() {
        let dto = build_dto(&[(-1, 5)]);
        let result = DirectedGraph::from_dto(&dto);
        assert!(matches!(result, DirectedGraphConstructionResult::ArrowOutsideOfNodesRange(_)));
    }

    #[test]
    fn test_arrows_out_of_range_2() {
        let dto = DirectedGraphDTO::new(1, Vec::from(&[ArrowDTO::new(0, 5)]));
        let result = DirectedGraph::from_dto(&dto);
        assert!(matches!(result, DirectedGraphConstructionResult::ArrowOutsideOfNodesRange(_)));
    }

    #[test]
    fn test_cycle() {
        let dto = build_dto(&[(0, 1), (1, 0)]);
        let result = DirectedGraph::from_dto(&dto);
        let graph = result.unwrap();
        assert_eq!(graph.get_number_of_nodes(), 2);
        let props = graph.get_basic_properties();
        assert!(!props.acyclic);
        assert!(props.connected);
        assert!(!props.rooted);
        for node in graph.iter_nodes() {
            assert_eq!(graph.get_successors(node).len(), 1);
            assert_eq!(graph.get_predecessors(node).len(), 1);
        }
    }

    #[test]
    fn test_bigger_cycle() {
        let dto = build_dto(&[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 0)]);
        let result = DirectedGraph::from_dto(&dto);
        let graph = result.unwrap();
        assert_eq!(graph.get_number_of_nodes(), 6);
        let props = graph.get_basic_properties();
        assert!(!props.acyclic);
        assert!(props.connected);
        assert!(!props.rooted);
        for node in graph.iter_nodes() {
            assert_eq!(graph.get_successors(node).len(), 1);
            assert_eq!(graph.get_predecessors(node).len(), 1);
        }
    }

    #[test]
    fn test_rooted_cycle() {
        let dto = build_dto(&[(0, 1), (1, 0), (2, 0)]);
        let result = DirectedGraph::from_dto(&dto);
        let graph = result.unwrap();
        assert_eq!(graph.get_number_of_nodes(), 3);
        let props = graph.get_basic_properties();
        assert!(!props.acyclic);
        assert!(props.connected);
        assert!(props.rooted);
    }

    
    #[test]
    fn test_disconnected_cycle() {
        let dto = build_dto(&[(0, 1), (1, 0), (2, 3), (3, 2)]);
        let result = DirectedGraph::from_dto(&dto);
        let graph = result.unwrap();
        assert_eq!(graph.get_number_of_nodes(), 4);
        let props = graph.get_basic_properties();
        assert!(!props.acyclic);
        assert!(!props.connected);
        assert!(!props.rooted);
    }

    #[test]
    fn test_binary() {
        let dto = build_dto(&[(0, 1), (1, 2), (1, 3), (2, 4)]);
        let result = DirectedGraph::from_dto(&dto);
        let graph = result.unwrap();
        assert_eq!(graph.get_number_of_nodes(), 5);
        let props = graph.get_basic_properties();
        assert!(props.acyclic);
        assert!(props.connected);
        assert!(props.rooted);
        assert!(graph.get_root().is_some_and(|val| val.get_numeric_id() == 0));
        let mut leaves = Vec::from_iter(graph.get_leaves().into_iter().map(|n| *n));
        leaves.sort_by_key(|n| n.get_numeric_id());
        assert_eq!(leaves.len(), 2);
        assert_eq!(leaves[0].get_numeric_id(), 3);
        assert_eq!(leaves[1].get_numeric_id(), 4);
    }

    #[test]
    fn test_with_reticulation() {
        let dto = build_dto(&[(0, 1), (1, 2), (1, 3), (2, 4), (3, 5), (2, 5)]);
        let result = DirectedGraph::from_dto(&dto);
        let graph = result.unwrap();
        assert_eq!(graph.get_number_of_nodes(), 6);
        let props = graph.get_basic_properties();
        assert!(props.acyclic);
        assert!(props.connected);
        assert!(props.rooted);
        assert!(graph.get_root().is_some_and(|val| val.get_numeric_id() == 0));
        let mut leaves = Vec::from_iter(graph.get_leaves().into_iter().map(|n| *n));
        leaves.sort_by_key(|n| n.get_numeric_id());
        assert_eq!(leaves.len(), 2);
        assert_eq!(leaves[0].get_numeric_id(), 4);
        assert_eq!(leaves[1].get_numeric_id(), 5);
    }
}
