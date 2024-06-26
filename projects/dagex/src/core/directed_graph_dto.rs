/// Represents arrow between source node and target node in a directed graph.
/// 
/// # Notes
/// Immutable once created.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub struct ArrowDTO {
    source: i32,
    target: i32,
}

impl ArrowDTO {
    #[inline(always)]
    pub fn new(source: i32, target: i32) -> Self {
        Self { source, target }
    }

    #[inline(always)]
    pub fn source(&self) -> i32 {
        self.source
    }

    #[inline(always)]
    pub fn target(&self) -> i32 {
        self.target
    }
}

/// Represents directed graph as a pair consisting of number of nodes,
/// and a collection of arrows.
/// 
/// # Notes
/// Immutable once created.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub struct DirectedGraphDTO {
    number_of_nodes: i32,
    arrows: Vec<ArrowDTO>,
}

impl DirectedGraphDTO {
    #[inline(always)]
    pub fn new(number_of_nodes: i32, arrows: Vec<ArrowDTO>) -> Self {
        Self { number_of_nodes, arrows }
    }

    #[inline(always)]
    pub fn number_of_nodes(&self) -> i32 {
        self.number_of_nodes
    }

    #[inline(always)]
    pub fn arrows(&self) -> &Vec<ArrowDTO> {
        &self.arrows
    }
}
