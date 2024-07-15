use smallvec::{Array, SmallVec};

#[doc(hidden)]
#[inline(always)]
pub fn empty_int_t_map<A: Array>() -> Vec<SmallVec<A>> {
    Vec::new()
}
