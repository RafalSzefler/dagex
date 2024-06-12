/// Represents arrow between source node and target node in a directed graph.
/// 
/// *Note:* it is immutable once created.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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
    pub fn get_source(&self) -> i32 {
        self.source
    }

    #[inline(always)]
    pub fn get_target(&self) -> i32 {
        self.target
    }
}

/// Represents directed graph as a pair consisting of number of nodes,
/// and a collection of arrows.
/// 
/// *Note:* it is immutable once created.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct DirectedGraphDTO {
    number_of_nodes: i32,
    arrows: Vec<ArrowDTO>,
}

impl DirectedGraphDTO {
    #[inline(always)]
    pub fn new(number_of_nodes: i32, arrows: Vec<ArrowDTO>) -> Self {
        Self { number_of_nodes, arrows }
    }

    pub fn get_number_of_nodes(&self) -> i32 {
        self.number_of_nodes
    }

    pub fn get_arrows(&self) -> &Vec<ArrowDTO> {
        &self.arrows
    }
}
