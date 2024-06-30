use immutable_string::ImmutableString;

use super::Id;


#[derive(PartialEq, Eq, Hash, Clone)]
pub enum NodeAttribute {
    Color(ImmutableString),
}

pub struct NodeAttributeGroup {
    id: Id<NodeAttributeGroup>,
    attributes: Vec<NodeAttribute>,
}

impl NodeAttributeGroup {
    #[inline(always)]
    pub unsafe fn new_unchecked(
        id: Id<NodeAttributeGroup>,
        attributes: Vec<NodeAttribute>) -> Self
    {
        Self {
            id: id,
            attributes: attributes,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> &Id<NodeAttributeGroup> { &self.id }

    #[inline(always)]
    pub fn iter_attributes(&self) -> impl Iterator<Item=&NodeAttribute> {
        self.attributes.iter()
    }
}
