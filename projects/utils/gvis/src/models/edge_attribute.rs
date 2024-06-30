use immutable_string::ImmutableString;

use super::Id;


#[derive(PartialEq, Eq, Hash, Clone)]
pub enum EdgeAttribute {
    Color(ImmutableString),
}

pub struct EdgeAttributeGroup {
    id: Id<EdgeAttributeGroup>,
    attributes: Vec<EdgeAttribute>,
}

impl EdgeAttributeGroup {
    #[inline(always)]
    pub unsafe fn new_unchecked(
        id: Id<EdgeAttributeGroup>,
        attributes: Vec<EdgeAttribute>) -> Self
    {
        Self {
            id: id,
            attributes: attributes,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> &Id<EdgeAttributeGroup> { &self.id }

    #[inline(always)]
    pub fn iter_attributes(&self) -> impl Iterator<Item=&EdgeAttribute> {
        self.attributes.iter()
    }
}
