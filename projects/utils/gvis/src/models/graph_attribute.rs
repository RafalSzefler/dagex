#[derive(PartialEq, Eq, Hash, Clone)]
pub enum GraphAttribute {
    SingleEdges,
    SingleArrows,
    MixedEdgesAndArrows,
}

pub struct GraphAttributeCollection {
    attributes: Vec<GraphAttribute>,
}

impl GraphAttributeCollection {
    #[inline(always)]
    pub unsafe fn new_unchecked(
        attributes: Vec<GraphAttribute>,
    ) -> Self {
        Self { attributes }
    }

    #[inline(always)]
    pub fn iter_attributes(&self) -> impl Iterator<Item=&GraphAttribute> {
        self.attributes.iter()
    }
}

impl Default for GraphAttributeCollection {
    fn default() -> Self {
        Self { attributes: Default::default() }
    }
}