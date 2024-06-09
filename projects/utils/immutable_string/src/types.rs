use core::sync::atomic::AtomicU32;

pub(crate) const MAX_PREFIX: usize = 1024;
pub(crate) type HashType = u32;
pub(crate) type LengthType = i32;
pub(crate) const MAX_LENGTH: usize
    = (LengthType::max_value() as usize) - MAX_PREFIX;

pub(crate) type AtomicType = AtomicU32;
pub(crate) type AtomicUnderlyingType = u32;
pub(crate) type DataType = u8;
pub(crate) type OffsetType = usize;
