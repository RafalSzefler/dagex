use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};

use smallvec::SmallVec;

use crate::create_u32_hasher;

use super::{ArrowDTO, DirectedGraphDTO, GraphId, Node};

type ArrowMap = Vec<SmallVec<[Node; 2]>>;

#[allow(clippy::struct_excessive_bools)]
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct DirectedGraphBasicProperties {
    /// The corresponding graph does not contain oriented cycles.
    pub acyclic: bool,

    /// The corresponding graph is connected in unoriented sense.
    pub connected: bool,

    /// The corresponding graph has a single node with in-degree 0, i.e.
    /// without arrows pointing to it.
    pub rooted: bool,

    /// Every node has at most two successors and at most two predecessors.
    pub binary: bool,

    /// Every node has at most one predecessor. Note that it might be
    /// counterintuitive if the graph is not acyclic.
    pub tree: bool,
}

/// Represents directed graph. The graph is expected to have a single arrow
/// between any two nodes, i.e. it is not a multigraph. Arrows in opposite
/// directions are allowed.
pub struct DirectedGraph {
    id: GraphId,
    number_of_nodes: i32,
    successors_map: ArrowMap,
    predecessors_map: ArrowMap,
    leaves: HashSet<Node>,
    root_node: Option<Node>,
    hash_value: u32,
    basic_properties: DirectedGraphBasicProperties,
}

static _EMPTY: &[Node] = &[];


impl DirectedGraph {
    #[inline(always)]
    pub const fn max_size() -> i32 {
        1 << 22
    }

    #[inline(always)]
    pub fn id(&self) -> GraphId {
        self.id
    }

    /// Retrieves total numbers of nodes in the graph.
    #[inline(always)]
    pub fn number_of_nodes(&self) -> i32 {
        self.number_of_nodes
    }

    #[inline(always)]
    pub fn iter_nodes(&self) -> impl Iterator<Item=Node> {
        (0..self.number_of_nodes).map(Node::from)
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
    pub fn basic_properties(&self) -> &DirectedGraphBasicProperties {
        &self.basic_properties
    }

    /// Returns the single node with in-degree 0 (i.e. without predecessors)
    /// if it exists.
    #[inline(always)]
    pub fn root(&self) -> Option<Node> {
        self.root_node
    }

    /// Returns all nodes with out-degree 0 (i.e. without successors).
    #[inline(always)]
    pub fn leaves(&self) -> &HashSet<Node> {
        &self.leaves
    }

    /// Checks if node is a leaf, i.e. of out-degree 0. This is an optimized
    /// version of `self.leaves().is_empty()`, it doesn't involve hash lookup.
    #[inline(always)]
    pub fn is_leaf(&self, node: Node) -> bool {
        self.get_successors(node).is_empty()
    }
    
    pub fn into_dto(&self) -> DirectedGraphDTO {
        let max_arrows = core::cmp::max(
            self.successors_map.len(),
            8);
        let mut arrows = Vec::<ArrowDTO>::with_capacity(max_arrows);
        for idx in 0..self.number_of_nodes {
            let node = Node::from(idx);
            for successor in self.get_successors(node) {
                let arrow = ArrowDTO::new(
                    node.id(), 
                    successor.id());
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
    let numeric_id = node.id();
    if numeric_id < 0 || numeric_id > (arrow_map.len() as i32) {
        _EMPTY
    }
    else
    {
        &arrow_map[numeric_id as usize]
    }
}


#[derive(Debug)]
pub enum DirectedGraphFromError {
    /// Passed graph didn't have nodes.
    EmptyGraph,

    /// Graph size exceeded [`DirectedGraph::max_size()`].
    TooBigGraph,

    /// Graph has multiple arrows between fixed (A, B) nodes. Returns the first
    /// conflicting arrow found.
    MultipleParallelArrows(ArrowDTO),

    /// Graph has arrows outside of range. Either with negative values, or
    /// with value exceeding the number of nodes. Returns the first
    /// conflicting arrow found.
    ArrowOutsideOfNodesRange(ArrowDTO),
}


impl DirectedGraph {
    /// Creates new [`DirectedGraph`] out of [`DirectedGraphDTO`].
    /// 
    /// # Errors
    /// For specific errors read [`DirectedGraphFromError`] docs.
    pub fn from_dto(value: &DirectedGraphDTO)
        -> Result<Self, DirectedGraphFromError>
    {
        let number_of_nodes = value.number_of_nodes();
        if number_of_nodes <= 0 {
            return Err(DirectedGraphFromError::EmptyGraph);
        }

        if number_of_nodes > Self::max_size() {
            return Err(DirectedGraphFromError::TooBigGraph);
        }

        let mut successor_map_duplicates 
            = HashMap::<Node, HashSet<Node>>::new();
        let mut predecessor_map_duplicates 
            = HashMap::<Node, HashSet<Node>>::new();
        let mut properties 
            = DirectedGraphBasicProperties {
                acyclic: false,
                connected: false,
                rooted: false,
                binary: true,
                tree: true,
            };
        let mut root_node = Option::<Node>::None;
        let mut multiple_roots = false;
        let mut leaves = HashSet::with_capacity(8);

        let arrows = value.arrows();
        let mut multi_arrows = HashSet::<ArrowDTO>::with_capacity(arrows.len());

        for arrow in arrows {
            if multi_arrows.contains(arrow) {
                return Err(DirectedGraphFromError::MultipleParallelArrows(arrow.clone()));
            }
            multi_arrows.insert(arrow.clone());
            let source = arrow.source();
            let target = arrow.target();
            if source < 0
                || source >= number_of_nodes
                || target < 0
                || target >= number_of_nodes
            {
                return Err(DirectedGraphFromError::ArrowOutsideOfNodesRange(arrow.clone()));
            }
            let source_node = Node::from(source);
            let target_node = Node::from(target);
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
            let node = Node::from(idx);
            let preds_len = predecessors_map[idx as usize].len();
            let succs_len = successors_map[idx as usize].len();
            if preds_len == 0 {
                if root_node.is_none() {
                    root_node = Some(node);
                }
                else
                {
                    multiple_roots = true;
                }
            }

            if succs_len == 0 {
                leaves.insert(node);
            }

            if preds_len > 2 || succs_len > 2 {
                properties.binary = false;
            }

            if preds_len > 1 {
                properties.tree = false;
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
            Self::new_unchecked(number_of_nodes, successors_map, predecessors_map, properties, root_node, leaves)
        };
        Ok(dg)
    }

    /// Creates an unchecked [`DirectedGraph`].
    /// 
    /// # Safety
    /// It is up to caller to ensure that all properties and invariants are
    /// satisfied and consistent. This also exposes internal structure of
    /// [`DirectedGraph`]. Use with caution. The following invariants have to
    /// be satisfied:
    /// * `number_of_nodes > 0`.
    /// * `successors_map` and `predecessors_map` are of length equalt
    ///   to `number_of_nodes`.
    /// * each value in `successors_map` and `predecessors_map` contains nodes within
    ///   `(0..number_of_nodes)` range.
    /// * each value in `successors_map` and `predecessors_map` is a vec ordered
    ///   by i32 representation of nodes. This is important for hash calculation.
    /// * acyclic, rooted and connected pieces of `properties` have to match the
    ///   actual graph structure.
    /// * `root_node` has to point to a single node without a predecessor. It
    ///   has to be `None` if either there is no root, or multiple are present.
    /// * `leaves` have to in `(0..number_of_nodes)` range, have to contain
    ///   nodes without successors, and have to be a complete list of such nodes
    ///   in the graph. The order is irrelevant.
    pub unsafe fn new_unchecked(
            number_of_nodes: i32,
            successors_map: Vec<SmallVec<[Node; 2]>>,
            predecessors_map: Vec<SmallVec<[Node; 2]>>,
            properties: DirectedGraphBasicProperties,
            root_node: Option<Node>,
            leaves: HashSet<Node>) -> Self
    {
        #[allow(clippy::cast_possible_truncation)]
        let hash = {
            fn update_vec<T: Hasher>(vec: &[SmallVec<[Node; 2]>], hasher: &mut T)
            {
                vec.len().hash(hasher);
                for (idx, internal) in vec.iter().enumerate() {
                    idx.hash(hasher);
                    internal.len().hash(hasher);
                    let mut res = 0;
                    for node in internal {
                        let mut internal_hasher = create_u32_hasher();
                        node.hash(&mut internal_hasher);
                        res ^= internal_hasher.finish();
                    }
                    res.hash(hasher);
                }
            }

            let mut hasher = create_u32_hasher();
            number_of_nodes.hash(&mut hasher);
            update_vec(&successors_map, &mut hasher);
            update_vec(&predecessors_map, &mut hasher);
            hasher.finish() as u32
        };

        Self {
            id: GraphId::generate_next(),
            number_of_nodes: number_of_nodes,
            successors_map: successors_map,
            predecessors_map: predecessors_map,
            basic_properties: properties,
            root_node: root_node,
            leaves: leaves,
            hash_value: hash,
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
        = (0..number_of_nodes).map(Node::from).collect();
    let first = Node::from(0);

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
    let idx = node.id() as usize;

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
        = (0..number_of_nodes).map(Node::from).collect();

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

    let succs = &successors_map[node.id() as usize];
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
        let node = &Node::from(idx);
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

    for internal in &mut result {
        internal.sort_by_key(Node::id);
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

impl PartialEq for DirectedGraph {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
        || (
            self.hash_value == other.hash_value
            && self.number_of_nodes == other.number_of_nodes
            && self.successors_map == other.successors_map
            && self.predecessors_map == other.predecessors_map)
    }
}

impl Eq for DirectedGraph { }

impl Hash for DirectedGraph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_value.hash(state);
    }
}

impl Clone for DirectedGraph {
    fn clone(&self) -> Self {
        unsafe {
            Self::new_unchecked(
                self.number_of_nodes,
                self.successors_map.clone(),
                self.predecessors_map.clone(),
                self.basic_properties.clone(),
                self.root_node,
                self.leaves.clone())
        }
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for DirectedGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirectedGraph")
            .field("id", &self.id)
            .field("number_of_nodes", &self.number_of_nodes)
            .field("hash_value", &self.hash_value)
            .field("successors_map", &self.successors_map)
            .field("predecessors_map", &self.predecessors_map)
            .finish()
    }
}

unsafe impl Sync for DirectedGraph { }
unsafe impl Send for DirectedGraph { }
