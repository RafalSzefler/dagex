const FNV1A_32_MULTIPLIER: u32 = 0x01000193;

pub fn update_fnv1a_32(current: &mut u32, bytes: &[u8]) {
    let mut tmp: u32 = *current;
    for val in bytes {
        tmp ^= u32::from(*val);
        tmp = tmp.wrapping_mul(FNV1A_32_MULTIPLIER);
    }
    *current = tmp;
}
