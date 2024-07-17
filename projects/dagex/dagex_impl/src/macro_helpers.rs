use smallvec::SmallVec;

use crate::core::Node;

#[doc(hidden)]
#[inline(always)]
pub fn empty_arow_map() -> Vec<SmallVec<[Node; 2]>> {
    Vec::new()
}


#[doc(hidden)]
#[inline(always)]
pub fn default_node() -> Node {
    Node::from(-1)
}
