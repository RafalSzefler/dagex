use core::hash::Hasher;

#[inline(always)]
pub(crate) fn create_u32_hasher() -> impl Hasher {
    raf_fnv1a_hasher::FNV1a32Hasher::new()
}
