use core::alloc::Layout;
use core::hash::{Hash, Hasher};
use core::mem::{align_of, size_of};
use core::ptr::null;
use core::slice;
use core::sync::atomic::Ordering;
use std::alloc::{alloc, dealloc};

use crate::types::{MAX_LENGTH, MAX_PREFIX};
use crate::{
    fnv1_hash::calculate_fnv1a_hash,
    layout_helpers::{extend_layout_for_type, max},
    types::{
        AtomicType,
        AtomicUnderlyingType,
        DataType,
        HashType,
        LengthType,
        OffsetType
    },
    ConstructionError
};


struct StringBufferLayout {
    alignment: usize,
    strong_offset: OffsetType,
    weak_offset: OffsetType,
    data_offset: OffsetType,
}

const fn get_immutable_string_layout() -> StringBufferLayout {
    assert!(size_of::<AtomicType>() == size_of::<AtomicUnderlyingType>(), "AtomicType and AtomicUnderlyingType have to have the same size.");
    assert!(align_of::<AtomicType>() == align_of::<AtomicUnderlyingType>(), "AtomicType and AtomicUnderlyingType have to have the same alignment.");

    let mut layout: Layout = match Layout::from_size_align(0, 1) {
        Ok(result_layout) => { result_layout }
        Err(_) => { panic!("Couldn't initalize layout") }
    };

    let alignment: usize;
    let strong_offset: OffsetType;
    let weak_offset: OffsetType;
    let data_offset: OffsetType;

    {
        let (new_layout, offset) 
            = extend_layout_for_type::<AtomicType>(&layout);
        layout = new_layout;
        strong_offset = offset;
    }

    {
        let (new_layout, offset) 
            = extend_layout_for_type::<AtomicType>(&layout);
        layout = new_layout;
        weak_offset = offset;
    }

    {
        let (_, offset)
            = extend_layout_for_type::<DataType>(&layout);
        data_offset = offset;
        assert!(data_offset <= MAX_PREFIX, "Data offset above 1024? Something definitely went wrong.");
    }

    {
        let mut a = align_of::<AtomicType>();
        a = max(a, align_of::<HashType>());
        a = max(a, align_of::<LengthType>());
        a = max(a, align_of::<DataType>());
        alignment = a;
        assert!(alignment <= MAX_PREFIX, "Alignment above 1024? Something definitely went wrong.");
        assert!(alignment.is_power_of_two(), "Expected alignment as power of two.");
    }

    StringBufferLayout {
        alignment: alignment,
        strong_offset: strong_offset,
        weak_offset: weak_offset,
        data_offset: data_offset,
    }
}

const _LAYOUT: StringBufferLayout = get_immutable_string_layout();

pub(crate) struct StringBuffer {
    length: LengthType,
    hash: HashType,
    raw_ptr: *mut u8,
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap)]
#[inline(always)]
fn _calculate_real_size(size: i32) -> i32 {
    const ALIGN: usize = _LAYOUT.alignment;
    let len = _LAYOUT.data_offset + (size as usize);
    ((len + ALIGN - 1) & !(ALIGN - 1)) as i32
}

impl StringBuffer {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        clippy::cast_ptr_alignment)]
    pub(crate) fn new(text: &str) -> Result<Self, ConstructionError> {
        let array_len = text.len();
        if array_len > MAX_LENGTH {
            return Err(ConstructionError::LengthTooBig);
        }
        let len = array_len as LengthType;
        let hash = calculate_fnv1a_hash(text.as_bytes());

        let total_length = _calculate_real_size(len) as usize;
        let layout = unsafe {
            Layout::from_size_align_unchecked(
                total_length, _LAYOUT.alignment)
        };
        let raw_ptr: *mut u8;
        unsafe {
            raw_ptr = alloc(layout);
            if core::ptr::eq(raw_ptr, null()) {
                return Err(ConstructionError::AllocationError);
            }

            // Zero prefix.
            {
                const PADDED_STRONG: bool = _LAYOUT.weak_offset != _LAYOUT.strong_offset + size_of::<AtomicType>();
                const PADDED_WEAK: bool = _LAYOUT.data_offset != _LAYOUT.weak_offset + size_of::<AtomicType>();
                const ZEROING_NEEDED: bool = PADDED_STRONG || PADDED_WEAK;
                if ZEROING_NEEDED {
                    raw_ptr.write_bytes(0, _LAYOUT.data_offset);
                }
            }

            // Fill struct
            *(raw_ptr.add(_LAYOUT.strong_offset).cast::<AtomicType>()) = AtomicType::new(1);
            *(raw_ptr.add(_LAYOUT.weak_offset).cast::<AtomicType>()) = AtomicType::new(1);

            let data_ptr = raw_ptr.add(_LAYOUT.data_offset);
            if array_len > 0 {
                data_ptr.copy_from_nonoverlapping(text.as_ptr(), array_len);
            }

            let padding_count = total_length - _LAYOUT.data_offset - array_len;
            if padding_count > 0 {
                // Zero the padding
                data_ptr.add(array_len).write_bytes(0, padding_count);
            }
        }

        return Ok(Self { length: len, hash: hash, raw_ptr: raw_ptr });
    }

    #[allow(clippy::cast_ptr_alignment)]
    #[inline(always)]
    pub(crate) fn get_strong_counter(&self) -> &AtomicType {
        unsafe {
            let ptr
                = self.raw_ptr.add(_LAYOUT.strong_offset)
                .cast::<AtomicUnderlyingType>();
            AtomicType::from_ptr(ptr)
        }
    }

    #[allow(clippy::cast_ptr_alignment)]
    #[inline(always)]
    pub(crate) fn get_weak_counter(&self) -> &AtomicType {
        unsafe {
            let ptr
                = self.raw_ptr.add(_LAYOUT.weak_offset)
                .cast::<AtomicUnderlyingType>();
            AtomicType::from_ptr(ptr)
        }
    }

    #[allow(clippy::cast_ptr_alignment)]
    #[inline(always)]
    pub(crate) fn len(&self) -> LengthType {
        self.length
    }

    #[allow(clippy::cast_sign_loss)]
    #[inline(always)]
    pub(crate) unsafe fn deallocate(&self) {
        let total_length =  _calculate_real_size(self.len()) as usize;
        let layout = Layout::from_size_align_unchecked(
            total_length, _LAYOUT.alignment);
        dealloc(self.raw_ptr, layout);
    }

    #[inline(always)]
    pub(crate) unsafe fn clone(&self) -> Self {
        Self { length: self.length, hash: self.hash, raw_ptr: self.raw_ptr }
    }

    #[inline(always)]
    pub(crate) fn inc_strong(&self) -> AtomicUnderlyingType {
        self.get_strong_counter().fetch_add(1, Ordering::Relaxed)
    }

    #[inline(always)]
    pub(crate) fn inc_weak(&self) -> AtomicUnderlyingType {
        self.get_weak_counter().fetch_add(1, Ordering::Relaxed)
    }

    #[inline(always)]
    pub(crate) fn dec_strong(&self) -> AtomicUnderlyingType {
        self.get_strong_counter().fetch_sub(1, Ordering::Release)
    }

    #[inline(always)]
    pub(crate) fn dec_weak(&self) -> AtomicUnderlyingType {
        self.get_weak_counter().fetch_sub(1, Ordering::Release)
    }

    #[allow(clippy::cast_sign_loss)]
    #[inline(always)]
    pub(crate) fn as_slice(&self) -> &[u8] {
        unsafe {
            let length = self.len();
            let ptr = self.raw_ptr.add(_LAYOUT.data_offset);
            slice::from_raw_parts(ptr, length as usize)
        }
    }

    #[allow(clippy::cast_ptr_alignment)]
    #[inline(always)]
    pub(crate) fn get_hash(&self) -> HashType {
        self.hash
    }

    #[cfg(test)]
    pub(crate) fn as_str(&self) -> &str {
        use core::str::from_utf8_unchecked;

        unsafe {
            from_utf8_unchecked(self.as_slice())
        }
    }

    #[cfg(test)]
    pub(crate) fn as_ptr(&self) -> *const u8 {
        self.raw_ptr
    }

    #[cfg(test)]
    pub(crate) fn get_buffer_size(&self) -> i32 {
        _calculate_real_size(self.len())
    }
}

unsafe impl Send for StringBuffer {}
unsafe impl Sync for StringBuffer {}

impl PartialEq for StringBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.get_hash() == other.get_hash()
            && self.as_slice() == other.as_slice()
    }
}

impl Eq for StringBuffer { }

impl Hash for StringBuffer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.get_hash());
    }
}


#[cfg(test)]
impl core::fmt::Debug for StringBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("StringBuffer")
            .field("content", &self.as_str())
            .finish()
    }
}

#[cfg(test)]
impl core::fmt::Display for StringBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}
