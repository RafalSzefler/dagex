use crate::raf_immutable_string::{ImmutableString, ConstructionError};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Taxon {
    value: ImmutableString,
}

impl Taxon {
    pub fn new(text: &str) -> Result<Self, ConstructionError> {
        let imm = ImmutableString::new(text)?;
        Ok(Self { value: imm })
    }

    #[inline(always)]
    pub fn value(&self) -> &ImmutableString {
        &self.value
    }
}

impl From<ImmutableString> for Taxon {
    #[inline(always)]
    fn from(value: ImmutableString) -> Self {
        Self { value: value }
    }
}
