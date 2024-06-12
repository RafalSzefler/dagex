use std::collections::{HashMap, HashSet};

use crate::core::{DirectedGraph, DirectedGraphFromResult, Node};

use super::{PhylogeneticNetworkDTO, PhylogeneticNetworkId, Taxon};


#[allow(clippy::struct_excessive_bools)]
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct PhylogeneticNetworkProperties {
    /// All leaves have taxon attached.
    pub all_leaves_labeled: bool,

    /// Each internal node has a child that has exactly one parent.
    pub tree_child: bool,
}

/// Represents phylogenetic network, which is a directed graph
/// with additional labels (taxons) on leaves.
#[derive(Clone)]
pub struct PhylogeneticNetwork {
    id: PhylogeneticNetworkId,
    graph: DirectedGraph,
    taxa: HashMap<Node, Taxon>,
    properties: PhylogeneticNetworkProperties,
}

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
    /// to not be `DirectedGraphConstructionResult::Ok`.
    GraphError(DirectedGraphFromResult),
}

impl PhylogeneticNetworkFromResult {
    /// Unwraps `PhyloConstructionResult::Ok` value.
    /// 
    /// # Panics
    /// Only and always when `self` is not `PhyloConstructionResult::Ok`.
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
    /// Constructs `PhylogeneticNetwork` directly.
    /// 
    /// # Safety
    /// This method is unsafe since it doesn't verify invariants:
    /// * `graph` has to be acyclic, rooted and binary.
    /// * `taxa` has to map leaves only.
    /// * `properties` has to be consistent with the graph.
    #[inline(always)]
    pub unsafe fn from_unchecked(
        id: PhylogeneticNetworkId,
        graph: DirectedGraph,
        taxa: HashMap<Node, Taxon>,
        properties: PhylogeneticNetworkProperties) -> Self
    {
        Self { id, graph, taxa, properties }
    }

    /// Safely constructs `PhylogeneticNetwork` directly and calculates/verifies
    /// all associated invariants and properties.
    /// 
    /// # Panics
    /// Should never panic, unless the associated graph is tree but not
    /// tree-child. This can only happen due to bug.
    pub fn from_id_graph_and_taxa(
        id: PhylogeneticNetworkId,
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

        let mut properties = PhylogeneticNetworkProperties {
            all_leaves_labeled: true,
            tree_child: true,
        };

        for leaf in graph.get_leaves() {
            if !taxa_nodes.remove(leaf) {
                properties.all_leaves_labeled = false;
            }
        }

        if !taxa_nodes.is_empty() {
            return PhylogeneticNetworkFromResult::TaxaNotLeaves(graph);
        }

        let root = graph.get_root().unwrap();
        for node in graph.iter_nodes() {
            if node == root {
                continue;
            }

            if properties.tree_child {
                if graph.get_successors(node).is_empty() {
                    continue;
                }

                let successor_is_tree = graph.get_successors(node)
                    .iter()
                    .any(|succ| graph.get_predecessors(*succ).len() == 1);
                if !successor_is_tree {
                    properties.tree_child = false;
                }
            }
        }

        assert!(!props.tree || properties.tree_child,
            "If it is tree, then it has to be tree child. Something went seriously wrong.");

        unsafe {
            PhylogeneticNetworkFromResult::Ok(
                Self::from_unchecked(id, graph, taxa, properties))
        }
    }

    pub fn from_graph_and_taxa(
        graph: DirectedGraph,
        taxa: HashMap<Node, Taxon>)
        -> PhylogeneticNetworkFromResult
    {
        let id = PhylogeneticNetworkId::generate_next();
        Self::from_id_graph_and_taxa(id, graph, taxa)
    }

    pub fn from_dto(dto: &PhylogeneticNetworkDTO) -> PhylogeneticNetworkFromResult {
        match DirectedGraph::from_dto(dto.get_graph()) {
            DirectedGraphFromResult::Ok(graph) => {
                let id = PhylogeneticNetworkId::from(dto.get_id());
                let taxa: HashMap<Node, Taxon>
                    = dto.get_taxa()
                        .iter()
                        .map(|kvp| (Node::from(*kvp.0), Taxon::from(kvp.1.clone())))
                        .collect();
                Self::from_id_graph_and_taxa(id, graph, taxa)
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

    #[inline(always)]
    pub fn get_properties(&self) -> &PhylogeneticNetworkProperties {
        &self.properties
    }

    /// Returns root of the `PhyologeneticNetwork`.
    /// 
    /// # Panics
    /// Only when the network is constructed in an unsafe way, i.e. when
    /// the underlying graph is not rooted.
    #[inline(always)]
    pub fn get_root(&self) -> Node {
        self.graph.get_root().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use immutable_string::ImmutableString;

    use crate::core::{ArrowDTO, DirectedGraphDTO};

    use super::*;

    fn imm(text: &str) -> ImmutableString { ImmutableString::get(text).unwrap() }

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

    #[test]
    fn test_empty() {
        let dto = PhylogeneticNetworkDTO::new(
            1,
            dg_dto_empty(),
            HashMap::new());
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhylogeneticNetworkFromResult::GraphError(DirectedGraphFromResult::EmptyGraph)));
    }

    #[test]
    fn test_empty_with_taxa() {
        let dto = PhylogeneticNetworkDTO::new(
            1,
            dg_dto_empty(),
            HashMap::from_iter([(1, imm("test"))]));
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhylogeneticNetworkFromResult::GraphError(DirectedGraphFromResult::EmptyGraph)));
    }

    #[test]
    fn test_taxa_not_leaves_1() {
        let dto = PhylogeneticNetworkDTO::new(
            1,
            DirectedGraphDTO::new(1, Vec::new()),
            HashMap::from_iter([(1, imm("test"))]));
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhylogeneticNetworkFromResult::TaxaNotLeaves(_)));
    }

    #[test]
    fn test_taxa_not_leaves_2() {
        let dto = PhylogeneticNetworkDTO::new(
            1,
            dg_dto(&[(0, 1)]),
            HashMap::from_iter([(0, imm("test2"))]));
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhylogeneticNetworkFromResult::TaxaNotLeaves(_)));
    }

    #[test]
    fn test_ok() {
        const ID: i32 = 17;
        let dto = PhylogeneticNetworkDTO::new(
            ID,
            dg_dto(&[(0, 1), (0, 2)]),
            HashMap::from_iter([(1, imm("a")), (2, imm("xyz"))]));
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhylogeneticNetworkFromResult::Ok(_)));
        let network = result.unwrap();
        let taxa = network.get_taxa();
        assert_eq!(taxa.len(), 2);
        assert_eq!(taxa.get(&Node::from(1)).unwrap().as_immutable_string(), &imm("a"));
        assert_eq!(taxa.get(&Node::from(2)).unwrap().as_immutable_string(), &imm("xyz"));

        let props = network.get_properties();
        assert!(props.all_leaves_labeled);
        assert!(props.tree_child);
        assert_eq!(network.get_id(), PhylogeneticNetworkId::from(ID));

        let graph = network.get_graph();
        let props = graph.get_basic_properties();
        assert!(props.acyclic);
        assert!(props.connected);
        assert!(props.rooted);
        assert!(props.tree);
        let root = graph.get_root().unwrap();
        assert_eq!(root.get_numeric_id(), 0);
        let network_root = network.get_root();
        assert_eq!(network_root, root);

        assert_eq!(graph.get_number_of_nodes(), 3);
        let node0 = Node::from(0);
        let node1 = Node::from(1);
        let node2 = Node::from(2);

        assert_eq!(root, node0);

        let mut root_successors = Vec::from(graph.get_successors(node0));
        root_successors.sort_by_key(|n| n.get_numeric_id());

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
            1,
            dg_dto(&[(0, 1), (0, 2)]),
            HashMap::new());
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhylogeneticNetworkFromResult::Ok(_)));
        let network = result.unwrap();

        let props = network.get_properties();
        assert!(!props.all_leaves_labeled);
        assert!(network.get_graph().get_basic_properties().tree);
        assert!(props.tree_child);
        assert_eq!(network.get_id().get_numeric_id(), 1);
    }

    #[test]
    fn test_tree_child() {
        let dto = PhylogeneticNetworkDTO::new(
            1,
            dg_dto(&[(0, 1), (0, 2), (1, 3), (1, 4), (2, 4), (2, 5)]),
            HashMap::new());
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhylogeneticNetworkFromResult::Ok(_)));
        let network = result.unwrap();

        let props = network.get_properties();
        assert!(!props.all_leaves_labeled);
        assert!(!network.get_graph().get_basic_properties().tree);
        assert!(props.tree_child);
        assert_eq!(network.get_id().get_numeric_id(), 1);
    }

}
