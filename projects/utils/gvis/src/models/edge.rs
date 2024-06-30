use std::hash::{Hash, Hasher};
use super::{EdgeAttributeGroup, Id, Node};

pub struct Edge {
    id: Id<Edge>,
    source: Id<Node>,
    target: Id<Node>,
    attribute_group_id: Option<Id<EdgeAttributeGroup>>,
}

impl Edge {
    #[inline(always)]
    pub unsafe fn new_unchecked(
            id: Id<Edge>,
            source: Id<Node>,
            target: Id<Node>,
            attribute_group_id: Option<Id<EdgeAttributeGroup>>) -> Self
    {
        Self { id, source, target, attribute_group_id }
    }

    #[inline(always)]
    pub fn id(&self) -> &Id<Edge> { &self.id }

    #[inline(always)]
    pub fn attribute_group_id(&self) -> &Option<Id<EdgeAttributeGroup>> {
        &self.attribute_group_id
    }

    pub fn ends(&self) -> (Id<Node>, Id<Node>) {
        let mut source = self.source.clone();
        let mut target = self.target.clone();
        if source.as_immutable_string().as_str() < target.as_immutable_string().as_str() {
            std::mem::swap(&mut source, &mut target);
        }
        (source, target)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Edge { }

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
