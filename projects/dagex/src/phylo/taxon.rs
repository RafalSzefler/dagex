use raf_immutable_string::ImmutableString;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Taxon {
    value: ImmutableString,
}

impl Taxon {
    #[inline(always)]
    pub fn value(&self) -> &ImmutableString {
        &self.value
    }
}

impl From<ImmutableString> for Taxon {
    #[inline(always)]
    fn from(value: ImmutableString) -> Self {
        Self { value }
    }
}
