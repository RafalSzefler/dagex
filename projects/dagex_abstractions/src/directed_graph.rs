use crate::Node;

pub trait DirectedGraph {
    fn get_nodes(&self) -> impl Iterator<Item=Node>;
    fn get_successors(&self, node: Node) -> impl Iterator<Item=Node>;
    fn get_predecessors(&self, node: Node) -> impl Iterator<Item=Node>;
}
