use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use immutable_string::ImmutableString;

pub struct Id<T> {
    value: ImmutableString,
    phantom: PhantomData<T>,
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Id<T> { }

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), phantom: PhantomData }
    }
}

impl<T> Id<T> {
    #[inline(always)]
    pub fn new(value: ImmutableString) -> Self {
        Self { value, phantom: PhantomData }
    }

    #[inline(always)]
    pub fn as_immutable_string(&self) -> &ImmutableString {
        &self.value
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl<T> Default for Id<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::new(ImmutableString::empty().clone())
    }
}
