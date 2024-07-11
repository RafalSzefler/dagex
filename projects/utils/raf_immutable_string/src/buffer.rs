use core::slice;
use core::hash::{Hash, Hasher};

use crate::{
    string_buffer::StringBuffer,
    types::{HashType, LengthType}
};


pub(crate) struct Buffer {
    raw_ptr: *const u8,
    length: LengthType,
    hash: HashType,
}

impl Buffer {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap)]
    #[inline(always)]
    fn as_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.raw_ptr, 
                self.length as usize)
        }
    }
}

unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}

impl PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash && self.as_slice() == other.as_slice()
    }
}

impl Eq for Buffer { }

impl Hash for Buffer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.hash);
    }
}

impl From<&str> for Buffer {
    /// Note: this is a very unsafe operation. The content of `value`
    /// is not copied into `Buffer`, but rather simply pointer is set
    /// to point to it. This allows us for fast lookup in `_CACHE` without
    /// the need of copy.
    /// 
    /// It is dev's responsibility to ensure that the returned `Buffer`
    /// instance doesn't outlive passed `value`.
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap)]
    fn from(value: &str) -> Self {
        let mut hasher = raf_fnv1a_hasher::FNV1a32Hasher::new();
        value.as_bytes().hash(&mut hasher);
        Self {
            raw_ptr: value.as_ptr(),
            length: value.len() as LengthType,
            hash: hasher.finish() as HashType,
        }
    }
}

impl From<&StringBuffer> for Buffer {
    /// Note: this is also an unsafe operation. The content of `value`
    /// is not copied into `Buffer`, but rather simply pointer is set
    /// to point to it. This also allows us for fast lookup in `_CACHE`
    /// without the need of copy.
    /// 
    /// It is dev's responsibility to ensure that the returned `Buffer`
    /// instance doesn't outlive passed `value`.
    fn from(value: &StringBuffer) -> Self {
        Self {
            raw_ptr: value.as_slice().as_ptr(),
            length: value.len(),
            hash: value.get_hash(),
        }
    }
}
