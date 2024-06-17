use crate::Bit;
use crate::macros::{position_bit_size_helper, unsafe_size_it};


#[derive(Debug)]
pub enum ConstructionError {
    PointerMisaligned,
}

pub struct TaggedPointer<T: ?Sized, const BIT_COUNT: usize> {
    raw_ptr: *mut T,
}

impl<T: ?Sized, const BIT_COUNT: usize> TaggedPointer<T, BIT_COUNT> {
    const MASK: usize = {
        assert!(BIT_COUNT >= 1, "BIT_COUNT has to be at least 1.");
        assert!(BIT_COUNT < (usize::BITS as usize), "BIT_COUNT has to be at most 8*size_of::<usize>().");

        (1usize << BIT_COUNT) - 1
    };

    /// Creates new instance of `TaggedPointer` out of raw pointer.
    /// 
    /// # Errors
    /// Returns `ConstructionError` if `raw_ptr` is not correctly aligned to
    /// `1 << BIT_COUNT` bytes.
    pub fn new(raw_ptr: *mut T) -> Result<Self, ConstructionError> {
        let repr_ptr = unsafe_size_it!(raw_ptr);
        if (*repr_ptr & Self::MASK) != 0 {
            Err(ConstructionError::PointerMisaligned)
        }
        else
        {
            Ok(Self { raw_ptr: raw_ptr })
        }
    }

    /// Creates new instance of `TaggedPointer` out of raw pointer.
    /// 
    /// # Safety
    /// It is up to caller to ensure that `raw_ptr` is aligned up to
    /// `1 << BIT_COUNT` bytes. The behaviour of `TaggedPointer` is
    /// undefined otherwise, and likely to break.
    pub unsafe fn new_unchecked(raw_ptr: *mut T) -> Self {
        Self { raw_ptr: raw_ptr }
    }

    /// Sets bit at `POSITION` to passed `Bit` value.
    /// 
    /// # Panics
    /// Only when `POSITION >= BIT_COUNT`
    pub fn set_n_bit<const POSITION: usize>(&mut self, bit: Bit) {
        position_bit_size_helper!(POSITION, BIT_COUNT);
        let repr_ptr = unsafe_size_it!(self.raw_ptr);
        let mut new_repr = *repr_ptr;
        new_repr &= !(1usize << POSITION);
        new_repr |= (bit.as_u8() as usize) << POSITION;
        *repr_ptr = new_repr;
    }

    /// Retrieves bit value at `POSITION`.
    /// 
    /// # Panics
    /// Only when `POSITION >= BIT_COUNT`
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_n_bit<const POSITION: usize>(&self) -> Bit {
        position_bit_size_helper!(POSITION, BIT_COUNT);
        let repr_ptr = unsafe_size_it!(self.raw_ptr);
        let bit = ((*repr_ptr >> POSITION) & 1) as u8;
        unsafe { Bit::new_unchecked(bit) }
    }

    pub fn as_ptr(&self) -> *const T {
        self.as_raw_ptr()
    }

    pub fn as_ptr_mut(&mut self) -> *mut T {
        self.as_raw_ptr()
    }

    /// Dereferences current `TaggedPointer` to `&T`.
    /// 
    /// # Safety
    /// This is an inherently unsafe operation, it does not check whether
    /// pointer is valid or not.
    pub unsafe fn deref(&self) -> &T {
        unsafe { &*self.as_raw_ptr() }
    }

    /// Dereferences current `TaggedPointer` to `&mut T`.
    /// 
    /// # Safety
    /// This is an inherently unsafe operation, it does not check whether
    /// pointer is valid or not.
    pub unsafe fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.as_raw_ptr() }
    }

    pub fn get_tag(&self) -> usize {
        let repr = unsafe_size_it!(self.raw_ptr);
        *repr & Self::MASK
    }

    #[inline(always)]
    fn as_raw_ptr(&self) -> *mut T {
        let copy = self.raw_ptr;
        let repr_ptr = unsafe_size_it!(copy);
        *repr_ptr = *repr_ptr & !Self::MASK;
        copy
    }
}

impl<T: ?Sized, const BIT_COUNT: usize> Clone for TaggedPointer<T, BIT_COUNT> {
    /// Creates a clone of current `TaggedPointer`. This is in fact a copy
    /// of `TaggedPointer`, and so no internal cloning actually happens.
    /// 
    /// # Safety
    /// Clone shares state with `self`, meaning caller has to be extra
    /// careful when using it.
    fn clone(&self) -> Self {
        Self { raw_ptr: self.raw_ptr }
    }
}
