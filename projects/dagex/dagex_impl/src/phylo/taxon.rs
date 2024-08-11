use crate::raf_array::immutable_string::{ImmutableString, NewImmutableStringError};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Taxon {
    value: ImmutableString,
}

impl Taxon {
    /// Constructs new [`Taxon`] out of `&str`.
    /// 
    /// # Errors
    /// [`NewImmutableStringError`] forwarded from [`ImmutableString::new()`].
    pub fn new(text: &str) -> Result<Self, NewImmutableStringError> {
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
