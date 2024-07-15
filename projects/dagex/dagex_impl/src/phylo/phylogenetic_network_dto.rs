use std::collections::HashMap;

use crate::raf_immutable_string::ImmutableString;

use crate::core::DirectedGraphDTO;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
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
    pub fn graph(&self) -> &DirectedGraphDTO {
        &self.graph
    }

    #[inline(always)]
    pub fn taxa(&self) -> &HashMap<i32, ImmutableString> {
        &self.taxa
    }
}
