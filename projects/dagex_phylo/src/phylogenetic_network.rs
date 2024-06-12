use std::collections::{HashMap, HashSet};

use dagex_core::{DirectedGraph, DirectedGraphConstructionResult, Node};

use crate::{PhylogeneticNetworkDTO, Taxon};

/// Represents phylogenetic network, which is a directed graph
/// with additional labels (taxons) on leaves.
pub struct PhylogeneticNetwork {
    graph: DirectedGraph,
    taxa: HashMap<Node, Taxon>,
    all_leaves_labeled: bool,
}

pub enum PhyloConstructionResult {
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
    GraphError(DirectedGraphConstructionResult),
}

impl PhyloConstructionResult {
    /// Unwraps `PhyloConstructionResult::Ok` value.
    /// 
    /// # Panics
    /// Only and always when `self` is not `PhyloConstructionResult::Ok`.
    #[inline(always)]
    pub fn unwrap(self) -> PhylogeneticNetwork {
        match self {
            PhyloConstructionResult::Ok(graph) => graph,
            _ => panic!("PhyloConstructionResult is not Ok."),
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
    #[inline(always)]
    pub unsafe fn from_unchecked(
        graph: DirectedGraph,
        taxa: HashMap<Node, Taxon>,
        all_leaves_labeled: bool) -> Self
    {
        Self { graph, taxa, all_leaves_labeled }
    }

    pub fn from_graph_and_taxa(graph: DirectedGraph, taxa: HashMap<Node, Taxon>)
        -> PhyloConstructionResult
    {
        let props = graph.get_basic_properties();
        if !props.acyclic {
            return PhyloConstructionResult::NotAcyclic(graph);
        }

        if !props.rooted {
            return PhyloConstructionResult::NotRooted(graph);
        }

        if !props.binary {
            return PhyloConstructionResult::NotBinary(graph);
        }

        let mut taxa_nodes: HashSet<Node> = taxa.keys().copied().collect();
        let mut leaves: HashSet<Node> = graph.get_leaves().iter().copied().collect();

        for leaf in graph.get_leaves() {
            taxa_nodes.remove(leaf);
            leaves.remove(leaf);
        }

        if !taxa_nodes.is_empty() {
            return PhyloConstructionResult::TaxaNotLeaves(graph);
        }

        let all_leaves_labeled = leaves.is_empty();

        let result = unsafe {
            Self::from_unchecked(graph, taxa, all_leaves_labeled)
        };

        return PhyloConstructionResult::Ok(result);
    }

    pub fn from_dto(dto: &PhylogeneticNetworkDTO) -> PhyloConstructionResult {
        match DirectedGraph::from_dto(dto.get_graph()) {
            DirectedGraphConstructionResult::Ok(graph) => {
                let taxa: HashMap<Node, Taxon>
                    = dto.get_taxa()
                        .iter()
                        .map(|kvp| (Node::from(*kvp.0), Taxon::from(kvp.1.clone())))
                        .collect();
                Self::from_graph_and_taxa(graph, taxa)
            },
            err => {
                PhyloConstructionResult::GraphError(err)
            }
        }
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
    pub fn all_leaves_are_labeled(&self) -> bool {
        self.all_leaves_labeled
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
    use dagex_core::{ArrowDTO, DirectedGraphDTO};
    use immutable_string::ImmutableString;

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
            dg_dto_empty(),
            HashMap::new());
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhyloConstructionResult::GraphError(DirectedGraphConstructionResult::EmptyGraph)));
    }

    #[test]
    fn test_empty_with_taxa() {
        let dto = PhylogeneticNetworkDTO::new(
            dg_dto_empty(),
            HashMap::from_iter([(1, imm("test"))]));
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhyloConstructionResult::GraphError(DirectedGraphConstructionResult::EmptyGraph)));
    }

    #[test]
    fn test_taxa_not_leaves_1() {
        let dto = PhylogeneticNetworkDTO::new(
            DirectedGraphDTO::new(1, Vec::new()),
            HashMap::from_iter([(1, imm("test"))]));
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhyloConstructionResult::TaxaNotLeaves(_)));
    }

    #[test]
    fn test_taxa_not_leaves_2() {
        let dto = PhylogeneticNetworkDTO::new(
            dg_dto(&[(0, 1)]),
            HashMap::from_iter([(0, imm("test2"))]));
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhyloConstructionResult::TaxaNotLeaves(_)));
    }

    #[test]
    fn test_ok() {
        let dto = PhylogeneticNetworkDTO::new(
            dg_dto(&[(0, 1), (0, 2)]),
            HashMap::from_iter([(1, imm("a")), (2, imm("xyz"))]));
        
        let result = PhylogeneticNetwork::from_dto(&dto);
        assert!(matches!(result, PhyloConstructionResult::Ok(_)));
        let network = result.unwrap();
        let taxa = network.get_taxa();
        assert_eq!(taxa.len(), 2);
        assert_eq!(taxa.get(&Node::from(1)).unwrap().as_immutable_string(), &imm("a"));
        assert_eq!(taxa.get(&Node::from(2)).unwrap().as_immutable_string(), &imm("xyz"));

        assert!(network.all_leaves_are_labeled());

        let graph = network.get_graph();
        let props = graph.get_basic_properties();
        assert!(props.acyclic);
        assert!(props.connected);
        assert!(props.rooted);
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

}
