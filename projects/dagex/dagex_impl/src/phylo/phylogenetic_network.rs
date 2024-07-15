use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use std::collections::HashMap;

use crate::core::{DirectedGraph, DirectedGraphFromResult, Node};
use crate::create_u32_hasher;

use super::{PhylogeneticNetworkDTO, PhylogeneticNetworkId, Taxon};

/// Represents phylogenetic network, which is a directed graph
/// with additional labels (taxons) on leaves.
pub struct PhylogeneticNetwork {
    graph: DirectedGraph,
    taxa: HashMap<Node, Taxon>,
    id: PhylogeneticNetworkId,
    hash_value: u32,
}


#[derive(Debug)]
pub enum PhylogeneticNetworkFromResult {
    /// Passed graph is a phylogenetic network. Consumes passed value.
    Ok(PhylogeneticNetwork),

    /// Passed graph is not acyclic. Returns passed value.
    NotAcyclic(DirectedGraph),

    /// Passed graph is not rooted. Returns passed value.
    NotRooted(DirectedGraph),

    /// Passed graph is not binary. Returns passed value.
    NotBinary(DirectedGraph),

    /// Leaves not of in-degree 1.
    LeavesNotOfInDegreeOne(DirectedGraph),

    /// Internal error of graph construction. The internal value is guaranteed
    /// to not be [`DirectedGraphFromResult::Ok`].
    GraphError(DirectedGraphFromResult),
}

impl PhylogeneticNetworkFromResult {
    /// Unwraps [`PhylogeneticNetworkFromResult::Ok`] value.
    /// 
    /// # Panics
    /// Only and always when `self` is not [`PhylogeneticNetworkFromResult::Ok`].
    #[inline(always)]
    pub fn unwrap(self) -> PhylogeneticNetwork {
        if let PhylogeneticNetworkFromResult::Ok(network) = self {
            network
        }
        else
        {
            let name = core::any::type_name::<PhylogeneticNetworkFromResult>();
            panic!("{name} not Ok.");
        }
    }
}

impl PhylogeneticNetwork {
    /// Constructs [`PhylogeneticNetwork`] directly.
    /// 
    /// # Safety
    /// This method is unsafe since it doesn't verify invariants:
    /// * `graph` has to be acyclic, rooted and binary.
    /// * leaves have to be of in-degree 1.
    /// * `taxa` has to map leaves only.
    /// * `taxa` cannot contain duplicate nodes.
    #[inline(always)]
    pub unsafe fn from_unchecked(
        graph: DirectedGraph,
        taxa: HashMap<Node, Taxon>) -> Self
    {
        let id = PhylogeneticNetworkId::generate_next();
        let hash_value: u32;

        {
            let mut hasher = create_u32_hasher();
            graph.hash(&mut hasher);
            let mut ordered: Vec<(Node, Taxon)> = taxa
                .iter()
                .map(|kvp| (*kvp.0, kvp.1.clone()))
                .collect();
            ordered.sort_by_key(|kvp| kvp.0.id());

            ordered.len().hash(&mut hasher);
            for (node, taxon) in ordered {
                node.hash(&mut hasher);
                taxon.hash(&mut hasher);
            }

            #[allow(clippy::cast_possible_truncation)]
            {
                hash_value = hasher.finish() as u32;
            }
        }

        Self { graph, taxa, id, hash_value }
    }

    /// Safely constructs [`PhylogeneticNetwork`] directly and
    /// calculates/verifies all associated invariants and properties.
    pub fn from_graph_and_taxa(
        graph: DirectedGraph,
        taxa: HashMap<Node, Taxon>)
        -> PhylogeneticNetworkFromResult
    {
        let props = graph.basic_properties();
        if !props.acyclic {
            return PhylogeneticNetworkFromResult::NotAcyclic(graph);
        }

        if !props.rooted {
            return PhylogeneticNetworkFromResult::NotRooted(graph);
        }

        if !props.binary {
            return PhylogeneticNetworkFromResult::NotBinary(graph);
        }

        for leaf in graph.leaves() {
            if graph.get_predecessors(*leaf).len() != 1 {
                return PhylogeneticNetworkFromResult::LeavesNotOfInDegreeOne(graph);
            }
        }

        unsafe {
            PhylogeneticNetworkFromResult::Ok(
                Self::from_unchecked(graph, taxa))
        }
    }

    pub fn from_dto(dto: &PhylogeneticNetworkDTO) -> PhylogeneticNetworkFromResult {
        match DirectedGraph::from_dto(dto.graph()) {
            DirectedGraphFromResult::Ok(graph) => {
                let taxa: HashMap<Node, Taxon>
                    = dto.taxa()
                        .iter()
                        .map(|kvp| (Node::from(*kvp.0), Taxon::from(kvp.1.clone())))
                        .collect();
                Self::from_graph_and_taxa(graph, taxa)
            },
            err => {
                PhylogeneticNetworkFromResult::GraphError(err)
            }
        }
    }

    #[inline(always)]
    pub fn id(&self) -> PhylogeneticNetworkId {
        self.id
    }

    #[inline(always)]
    pub fn graph(&self) -> &DirectedGraph {
        &self.graph
    }

    #[inline(always)]
    pub fn taxa(&self) -> &HashMap<Node, Taxon> {
        &self.taxa
    }

    /// Returns root of the [`PhylogeneticNetwork`].
    /// 
    /// # Panics
    /// Only when the network is constructed in an unsafe way, i.e. when
    /// the underlying graph is not rooted.
    #[inline(always)]
    pub fn root(&self) -> Node {
        self.graph.root().unwrap()
    }

    /// Tree node is a node of in-degree at most 1, but is not a leaf.
    pub fn is_tree_node(&self, node: Node) -> bool {
        let graph = self.graph();
        if graph.leaves().contains(&node) {
            false
        }
        else
        {
            graph.get_predecessors(node).len() <= 1
        }
    }

    /// Reticulation node is a node of in-degree 2 and out-degree 1.
    #[inline(always)]
    pub fn is_reticulation_node(&self, node: Node) -> bool {
        let graph = self.graph();
        (graph.get_predecessors(node).len() == 2) && (graph.get_successors(node).len() == 1)
    }

    /// Cross node is a node of in-degree 2 and out-degree 2.
    #[inline(always)]
    pub fn is_cross_node(&self, node: Node) -> bool {
        let graph = self.graph();
        (graph.get_predecessors(node).len() == 2) && (graph.get_successors(node).len() == 2)
    }

    /// Leaf is a node of out-degree 0.
    #[inline(always)]
    pub fn is_leaf(&self, node: Node) -> bool {
        self.graph.is_leaf(node)
    }
}

impl PartialEq for PhylogeneticNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
        || (
            self.hash_value == other.hash_value
            && self.graph == other.graph
            && self.taxa == other.taxa)
    }
}

impl Eq for PhylogeneticNetwork { }

impl Hash for PhylogeneticNetwork {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_value.hash(state);
    }
}

impl Clone for PhylogeneticNetwork {
    fn clone(&self) -> Self {
        unsafe {
            Self::from_unchecked(
                self.graph.clone(),
                self.taxa.clone())
        }
    }
}

#[allow(clippy::missing_fields_in_debug)]
impl Debug for PhylogeneticNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let graph_id = self.graph().id();
        f.debug_struct("PhylogeneticNetwork")
            .field("id", &i32::from(self.id))
            .field("graph_id", &i32::from(graph_id))
            .field("hash_value", &self.hash_value)
            .finish()
    }
}

unsafe impl Sync for PhylogeneticNetwork { }
unsafe impl Send for PhylogeneticNetwork { }
