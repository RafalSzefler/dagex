use std::collections::HashMap;

use immutable_string::ImmutableString;

use crate::models::{Arrow, ArrowAttributeGroup, Edge, EdgeAttributeGroup, Graph, GraphAttributeCollection, Id, Node, NodeAttributeGroup};

#[derive(Default)]
pub struct GraphBuilder {
    id: Id<Graph>,
    graph_attributes: GraphAttributeCollection,
    nodes: Vec<Node>,
    node_attribute_groups: HashMap<Id<NodeAttributeGroup>, NodeAttributeGroup>,
    edges: Vec<Edge>,
    edge_attribute_groups: HashMap<Id<EdgeAttributeGroup>, EdgeAttributeGroup>,
    arrows: Vec<Arrow>,
    arrow_attribute_groups: HashMap<Id<ArrowAttributeGroup>, ArrowAttributeGroup>,
}

pub enum BuildError {

}

impl GraphBuilder {
    pub fn set_id(&mut self, name: &str) -> &mut Self {
        self.id = Id::new(ImmutableString::new(name.trim()).unwrap());
        self
    }

    pub fn build(self) -> Result<Graph, BuildError> {
        self.validate()?;

        let graph = unsafe {
            Graph::new_unchecked(
                self.id,
                self.graph_attributes,
                self.nodes,
                self.node_attribute_groups,
                self.edges,
                self.edge_attribute_groups,
                self.arrows,
                self.arrow_attribute_groups)
        };

        Ok(graph)
    }
    
    fn validate(&self) -> Result<(), BuildError> {
        Ok(())
    }
}
