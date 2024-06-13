pub(crate) const FNV1A_32_INITIAL: u32 = 0x811c9dc5;

#[cfg(feature="use-getrandom")]
mod rand_impl {
    use crate::update_fnv1a_32;

    use super::FNV1A_32_INITIAL;

    fn create_random_u32_state() -> u32 {
        const RANDOM_STATE_SIZE: usize = 32;
        let mut array: [u8; RANDOM_STATE_SIZE] = [0; RANDOM_STATE_SIZE];
        getrandom::getrandom(&mut array).unwrap();
        
        let mut hash = FNV1A_32_INITIAL;
        update_fnv1a_32(&mut hash, &array);
        return hash;
    }

    #[cfg(feature="use-ctor")]
    mod initial_impl {
        use ctor::ctor;

        use super::create_random_u32_state;

        static mut RANDOM_U32_STATE: u32 = 0;

        #[ctor]
        fn initialize_random_state() {
            unsafe {
                RANDOM_U32_STATE = create_random_u32_state();
            }
        }

        #[inline(always)]
        pub(super) fn get() -> u32 { unsafe { RANDOM_U32_STATE } }

    }

    #[cfg(not(feature="use-ctor"))]
    mod initial_impl {
        use std::sync::OnceLock;

        use super::create_random_u32_state;

        static RANDOM_U32_STATE: OnceLock<u32> = OnceLock::new();

        #[inline(always)]
        pub(super) fn get() -> u32 { *RANDOM_U32_STATE.get_or_init(create_random_u32_state) }
    }

    #[inline(always)]
    pub(super) fn init_32_hash() -> u32 {
        initial_impl::get()
    }
}

#[cfg(not(feature="use-getrandom"))]
mod rand_impl {
    use super::FNV1A_32_INITIAL;

    #[inline(always)]
    pub(super) fn init_32_hash() -> u32 { FNV1A_32_INITIAL }
}

#[inline(always)]
pub(crate) fn get_initial_32_hash() -> u32 {
    rand_impl::init_32_hash()
}
