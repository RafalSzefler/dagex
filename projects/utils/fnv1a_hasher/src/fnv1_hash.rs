use crate::types::HashType;

const FNV1A_INITIAL: u32 = 0x811c9dc5;
const FNV1A_MULTIPLIER: u32 = 0x01000193;

fn update_fnv1a_hash(hash: &mut HashType, array: &[u8]) {
    let mut tmp = *hash;
    for val in array {
        tmp ^= u32::from(*val);
        tmp = tmp.wrapping_mul(FNV1A_MULTIPLIER);
    }
    *hash = tmp;
}


#[cfg(feature="use-getrandom")]
mod rand_impl {
    use crate::{fnv1_hash::FNV1A_INITIAL, types::HashType};

    use super::update_fnv1a_hash;

    fn create_random_state() -> HashType {
        const RANDOM_STATE_SIZE: usize = 32;
        let mut array: [u8; RANDOM_STATE_SIZE] = [0; RANDOM_STATE_SIZE];
        getrandom::getrandom(&mut array).unwrap();
        
        let mut hash = FNV1A_INITIAL;
        update_fnv1a_hash(&mut hash, &array);
        return hash;
    }

    #[cfg(feature="use-ctor")]
    mod initial_impl {
        use ctor::ctor;

        use crate::types::HashType;

        use super::create_random_state;

        static mut RANDOM_STATE: HashType = 0;

        #[ctor]
        fn initialize_random_state() {
            unsafe {
                RANDOM_STATE = create_random_state();
            }
        }

        #[inline(always)]
        pub(super) fn get() -> HashType { unsafe { RANDOM_STATE } }

    }

    #[cfg(not(feature="use-ctor"))]
    mod initial_impl {
        use std::sync::OnceLock;

        use crate::types::HashType;

        use super::create_random_state;

        static RANDOM_STATE: OnceLock<HashType> = OnceLock::new();

        #[inline(always)]
        pub(super) fn get() -> HashType { *RANDOM_STATE.get_or_init(create_random_state) }
    }

    #[inline(always)]
    pub(super) fn init_hash() -> HashType {
        initial_impl::get()
    }
}

#[cfg(not(feature="use-getrandom"))]
mod rand_impl {
    use crate::types::HashType;

    use super::FNV1A_INITIAL;

    #[inline(always)]
    pub(super) fn init_hash() -> HashType { FNV1A_INITIAL }
}

pub(crate) fn calculate_fnv1a_hash(array: &[u8]) -> HashType {
    if array.is_empty() {
        return 0;
    }

    let mut hash = rand_impl::init_hash();
    update_fnv1a_hash(&mut hash, array);
    return hash;
}
