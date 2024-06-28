use core::hash::{Hash, Hasher};
use std::{alloc::Layout, ptr::{self, null_mut}};

/// Represents a dynamically created array with length known at runtime.
/// Otherwise a thin wrapper around slices.
pub struct Array<T>
    where T: Sized
{
    ptr: *mut T,
    length: usize,
}

impl<T> Array<T>
    where T: Sized
{
    const ALIGNEMENT: usize = {
        let alignement = core::mem::align_of::<T>();
        assert!(alignement.is_power_of_two(), "Alignement is not power of two.");
        alignement
    };

    pub const ALIGNED_SIZE: usize = {
        let result = {
            let size = core::mem::size_of::<T>();
            let align = Self::ALIGNEMENT;
            if (size % align) == 0 {
                size
            }
            else
            {
                let len_rounded_up 
                    = size.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);
                len_rounded_up.wrapping_sub(size)
            }
        };
        assert!(result.is_power_of_two(), "Aligned size not power of two.");
        result
    };

    pub const fn max() -> usize { (i32::MAX - 1024) as usize }

    const fn layout(length: usize) -> Layout {
        unsafe {
            Layout::from_size_align_unchecked(
                length * Self::ALIGNED_SIZE,
                Self::ALIGNEMENT)
        }
    }

    /// Creates a new instance of [`Array`]. It allocates the corresponding
    /// buffer on heap and fills it with values generated through `factory`.
    /// 
    /// # Panics
    /// Only when `length` is bigger than [`Self::max()`].
    pub fn new_with_fill<F: Fn()->T>(length: usize, factory: F) -> Self {
        assert!(length < Self::max(), "Length must be smaller than {}.", Self::max());

        if length == 0 {
            return Self::default()
        }

        let layout = Self::layout(length);
        let buffer = (unsafe { std::alloc::alloc_zeroed(layout) }).cast::<T>();
        for idx in 0..length {
            let piece = unsafe { buffer.add(idx) };
            unsafe { *piece = factory() };
        }
        Self { ptr: buffer, length: length }
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            &*ptr::slice_from_raw_parts(self.ptr, self.length)
        }
    }

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe {
            &mut *ptr::slice_from_raw_parts_mut(self.ptr, self.length)
        }
    }
}

impl<T> Array<T>
    where T: Sized + Default
{
    /// Creates a new instance of [`Array`]. It allocates the corresponding
    /// buffer on heap and fills it with `T::default()`.
    /// 
    /// # Panics
    /// Only when `length` is bigger than [`Self::max()`].
    pub fn new(length: usize) -> Self {
        Self::new_with_fill(length, T::default)
    }
}

impl<T> Default for Array<T>
    where T: Sized
{
    fn default() -> Self {
        Self { ptr: null_mut(), length: 0 }
    }
}

impl<T> Drop for Array<T>
    where T: Sized
{
    fn drop(&mut self) {
        let length = self.length;
        if length == 0 {
            return;
        }

        let layout = Self::layout(length);
        let raw_ptr = self.ptr.cast::<u8>();
        unsafe { std::alloc::dealloc(raw_ptr, layout) };
        self.ptr = null_mut();
        self.length = 0;
    }
}

impl<T> core::fmt::Debug for Array<T>
    where T: Sized
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let numeric_value = core::ptr::addr_of!(self.ptr).cast::<usize>();
        f.debug_struct("Array")
            .field("address", &numeric_value)
            .field("length", &self.length)
            .finish()
    }
}


impl<T> PartialEq for Array<T>
    where T: Sized + PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}


impl<T> Eq for Array<T>
    where T: Sized + Eq
{ }


impl<T> Hash for Array<T>
    where T: Sized + Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}
