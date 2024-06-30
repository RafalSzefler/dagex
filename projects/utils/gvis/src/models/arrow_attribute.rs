use immutable_string::ImmutableString;

use super::Id;


#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ArrowAttribute {
    Color(ImmutableString),
}

impl ArrowAttribute {
    pub fn tag(&self) -> i32 {
        match self {
            ArrowAttribute::Color(_) => 0,
        }
    }
}

pub struct ArrowAttributeGroup {
    id: Id<ArrowAttributeGroup>,
    attributes: Vec<ArrowAttribute>,
}

impl ArrowAttributeGroup {
    #[inline(always)]
    pub unsafe fn new_unchecked(
        id: Id<ArrowAttributeGroup>,
        attributes: Vec<ArrowAttribute>) -> Self
    {
        Self {
            id: id,
            attributes: attributes,
        }
    }

    #[inline(always)]
    pub fn id(&self) -> &Id<ArrowAttributeGroup> { &self.id }

    #[inline(always)]
    pub fn iter_attributes(&self) -> impl Iterator<Item=&ArrowAttribute> {
        self.attributes.iter()
    }
}
