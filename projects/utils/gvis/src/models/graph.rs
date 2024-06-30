use std::collections::HashMap;

use super::{Arrow, ArrowAttributeGroup, Edge, EdgeAttributeGroup, GraphAttributeCollection, Id, Node, NodeAttributeGroup};

pub struct Graph {
    id: Id<Graph>,
    graph_attributes: GraphAttributeCollection,
    nodes: Vec<Node>,
    node_attribute_groups: HashMap<Id<NodeAttributeGroup>, NodeAttributeGroup>,
    edges: Vec<Edge>,
    edge_attribute_groups: HashMap<Id<EdgeAttributeGroup>, EdgeAttributeGroup>,
    arrows: Vec<Arrow>,
    arrow_attribute_groups: HashMap<Id<ArrowAttributeGroup>, ArrowAttributeGroup>,
}

impl Graph {
    #[inline(always)]
    pub fn id(&self) -> &Id<Graph> { &self.id }

    #[inline(always)]
    pub fn graph_attributes(&self) -> &GraphAttributeCollection {
        &self.graph_attributes
    }

    #[inline(always)]
    pub fn iter_nodes(&self) -> impl Iterator<Item=&Node> {
        self.nodes.iter()
    }

    #[inline(always)]
    pub fn iter_node_attribute_groups(&self) -> impl Iterator<Item=&NodeAttributeGroup> {
        self.node_attribute_groups.values()
    }

    #[inline(always)]
    pub fn get_node_attribute_group(&self, id: &Id<NodeAttributeGroup>) -> Option<&NodeAttributeGroup> {
        self.node_attribute_groups.get(id)
    }

    #[inline(always)]
    pub fn iter_edges(&self) -> impl Iterator<Item=&Edge> {
        self.edges.iter()
    }

    #[inline(always)]
    pub fn iter_edge_attribute_groups(&self) -> impl Iterator<Item=&EdgeAttributeGroup> {
        self.edge_attribute_groups.values()
    }

    #[inline(always)]
    pub fn get_edge_attribute_group(&self, id: &Id<EdgeAttributeGroup>) -> Option<&EdgeAttributeGroup> {
        self.edge_attribute_groups.get(id)
    }

    #[inline(always)]
    pub fn iter_arrows(&self) -> impl Iterator<Item=&Arrow> {
        self.arrows.iter()
    }

    #[inline(always)]
    pub fn iter_arrow_attribute_groups(&self) -> impl Iterator<Item=&ArrowAttributeGroup> {
        self.arrow_attribute_groups.values()
    }

    #[inline(always)]
    pub fn get_arrow_attribute_group(&self, id: &Id<ArrowAttributeGroup>) -> Option<&ArrowAttributeGroup> {
        self.arrow_attribute_groups.get(id)
    }

    #[inline(always)]
    pub unsafe fn new_unchecked(
        id: Id<Graph>,
        graph_attributes: GraphAttributeCollection,
        nodes: Vec<Node>,
        node_attribute_groups: HashMap<Id<NodeAttributeGroup>, NodeAttributeGroup>,
        edges: Vec<Edge>,
        edge_attribute_groups: HashMap<Id<EdgeAttributeGroup>, EdgeAttributeGroup>,
        arrows: Vec<Arrow>,
        arrow_attribute_groups: HashMap<Id<ArrowAttributeGroup>, ArrowAttributeGroup>,
    ) -> Self {
        Self {
            id,
            graph_attributes,
            nodes,
            node_attribute_groups,
            edges,
            edge_attribute_groups,
            arrows,
            arrow_attribute_groups,
        }
    }
}
