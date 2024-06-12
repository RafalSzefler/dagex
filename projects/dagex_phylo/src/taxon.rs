use immutable_string::ImmutableString;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Taxon {
    value: ImmutableString,
}

impl Taxon {
    pub fn as_immutable_string(&self) -> &ImmutableString {
        &self.value
    }
}

impl From<ImmutableString> for Taxon {
    fn from(value: ImmutableString) -> Self {
        Self { value }
    }
}
