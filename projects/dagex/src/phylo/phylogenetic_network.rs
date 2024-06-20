use core::fmt::{Debug, Formatter};
use core::hash::{Hash, Hasher};
use std::collections::{HashMap, HashSet};

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

    /// Taxa map contains nodes that are not leaves. Returns passed value.
    TaxaNotLeaves(DirectedGraph),

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
            ordered.sort_by_key(|kvp| kvp.0.as_i32());

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
        let props = graph.get_basic_properties();
        if !props.acyclic {
            return PhylogeneticNetworkFromResult::NotAcyclic(graph);
        }

        if !props.rooted {
            return PhylogeneticNetworkFromResult::NotRooted(graph);
        }

        if !props.binary {
            return PhylogeneticNetworkFromResult::NotBinary(graph);
        }

        let mut taxa_nodes: HashSet<Node> = taxa.keys().copied().collect();

        for leaf in graph.get_leaves() {
            taxa_nodes.remove(leaf);
        }

        if !taxa_nodes.is_empty() {
            return PhylogeneticNetworkFromResult::TaxaNotLeaves(graph);
        }

        unsafe {
            PhylogeneticNetworkFromResult::Ok(
                Self::from_unchecked(graph, taxa))
        }
    }

    pub fn from_dto(dto: &PhylogeneticNetworkDTO) -> PhylogeneticNetworkFromResult {
        match DirectedGraph::from_dto(dto.get_graph()) {
            DirectedGraphFromResult::Ok(graph) => {
                let taxa: HashMap<Node, Taxon>
                    = dto.get_taxa()
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
    pub fn get_id(&self) -> PhylogeneticNetworkId {
        self.id
    }

    #[inline(always)]
    pub fn get_graph(&self) -> &DirectedGraph {
        &self.graph
    }

    #[inline(always)]
    pub fn get_taxa(&self) -> &HashMap<Node, Taxon> {
        &self.taxa
    }

    /// Returns root of the [`PhylogeneticNetwork`].
    /// 
    /// # Panics
    /// Only when the network is constructed in an unsafe way, i.e. when
    /// the underlying graph is not rooted.
    #[inline(always)]
    pub fn get_root(&self) -> Node {
        self.graph.get_root().unwrap()
    }
}

impl PartialEq for PhylogeneticNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.hash_value == other.hash_value
            && self.taxa == other.taxa
            && self.graph == other.graph
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
        let graph_id = self.get_graph().get_id();
        f.debug_struct("PhylogeneticNetwork")
            .field("id", &i32::from(self.id))
            .field("graph_id", &i32::from(graph_id))
            .field("hash_value", &self.hash_value)
            .finish()
    }
}

unsafe impl Sync for PhylogeneticNetwork { }
unsafe impl Send for PhylogeneticNetwork { }
