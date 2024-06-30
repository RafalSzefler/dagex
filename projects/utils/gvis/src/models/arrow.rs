use std::hash::{Hash, Hasher};
use super::{ArrowAttributeGroup, Id, Node};

pub struct Arrow {
    id: Id<Arrow>,
    source: Id<Node>,
    target: Id<Node>,
    attribute_group_id: Option<Id<ArrowAttributeGroup>>,
}

impl Arrow {
    #[inline(always)]
    pub unsafe fn new_unchecked(
            id: Id<Arrow>,
            source: Id<Node>,
            target: Id<Node>,
            attribute_group_id: Option<Id<ArrowAttributeGroup>>) -> Self
    {
        Self { id, source, target, attribute_group_id }
    }

    #[inline(always)]
    pub fn id(&self) -> &Id<Arrow> { &self.id }

    #[inline(always)]
    pub fn attribute_group_id(&self) -> &Option<Id<ArrowAttributeGroup>> {
        &self.attribute_group_id
    }

    #[inline(always)]
    pub fn source(&self) -> &Id<Node> {
        &self.source
    }

    #[inline(always)]
    pub fn target(&self) -> &Id<Node> {
        &self.target
    }
}

impl PartialEq for Arrow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Arrow { }

impl Hash for Arrow {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
