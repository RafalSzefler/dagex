use core::hash::Hasher;

use crate::{init, update_fnv1a_32};

pub struct FNV1a32Hasher {
    current_value: u32,
}

impl Hasher for FNV1a32Hasher {

    #[allow(clippy::cast_lossless)]
    fn finish(&self) -> u64 {
        self.current_value as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        update_fnv1a_32(
            &mut self.current_value,
            bytes);
    }
}

impl FNV1a32Hasher {
    pub fn new() -> Self {
        let initial_value = init::get_initial_32_hash();
        Self { current_value: initial_value }
    }
}

impl Default for FNV1a32Hasher {
    fn default() -> Self {
        Self { current_value: init::FNV1A_32_INITIAL }
    }
}