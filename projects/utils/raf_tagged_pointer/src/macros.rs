macro_rules! unsafe_size_it {
    ( $x: expr ) => {
        { 
            let internal_ptr = core::ptr::addr_of!($x) as *mut usize;
            let internal_ref = unsafe { &mut *internal_ptr };
            internal_ref as &mut usize
        }
    };
}

pub(crate) use unsafe_size_it;

macro_rules! position_bit_size_helper {
    ( $x: expr, $y: expr ) => {
        {
            struct __Helper<const __TPOS: usize, const __TBIT: usize>();
            impl<const __TPOS: usize, const __TBIT: usize> __Helper<__TPOS, __TBIT> {
                const ASSERTION: () = assert!(__TPOS < __TBIT, "POSITION outside of BIT_COUNT range.");
            }
            let _ = __Helper::<$x, $y>::ASSERTION;
        }
    };
}

pub(crate) use position_bit_size_helper;
