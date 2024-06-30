use std::hash::{Hash, Hasher};
use super::{Id, NodeAttributeGroup};

pub struct Node {
    id: Id<Node>,
    attribute_group_id: Option<Id<NodeAttributeGroup>>,
}

impl Node {
    #[inline(always)]
    pub unsafe fn new_unchecked(
        id: Id<Node>,
        attribute_group_id: Option<Id<NodeAttributeGroup>>
    ) -> Self {
        Self { id, attribute_group_id }
    }

    #[inline(always)]
    pub fn id(&self) -> &Id<Node> { &self.id }

    #[inline(always)]
    pub fn attribute_group_id(&self) -> &Option<Id<NodeAttributeGroup>> { &self.attribute_group_id }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node { }

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
