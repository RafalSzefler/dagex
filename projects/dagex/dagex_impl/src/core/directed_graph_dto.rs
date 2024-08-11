use raf_readonly::readonly;

/// Represents arrow between source node and target node in a directed graph.
/// 
/// # Notes
/// Immutable once created.
#[readonly]
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub struct ArrowDTO {
    pub source: i32,
    pub target: i32,
}

/// Represents directed graph as a pair consisting of number of nodes,
/// and a collection of arrows.
/// 
/// # Notes
/// Immutable once created.
#[readonly]
#[derive(PartialEq, Eq, Hash, Clone, Debug, Default)]
pub struct DirectedGraphDTO {
    pub number_of_nodes: i32,
    pub arrows: Vec<ArrowDTO>,
}
