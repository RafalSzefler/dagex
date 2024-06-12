use std::collections::HashMap;

use dagex_core::DirectedGraphDTO;
use immutable_string::ImmutableString;

pub struct PhylogeneticNetworkDTO {
    graph: DirectedGraphDTO,
    taxa: HashMap<i32, ImmutableString>,
}

impl PhylogeneticNetworkDTO {
    #[inline(always)]
    pub fn new(graph: DirectedGraphDTO, taxa: HashMap<i32, ImmutableString>) -> Self {
        Self { graph, taxa }
    }

    #[inline(always)]
    pub fn get_graph(&self) -> &DirectedGraphDTO {
        &self.graph
    }

    #[inline(always)]
    pub fn get_taxa(&self) -> &HashMap<i32, ImmutableString> {
        &self.taxa
    }
}

