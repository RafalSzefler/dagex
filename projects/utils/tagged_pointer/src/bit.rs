use std::ops::{BitAnd, BitOr, BitXor};

/// Represents two bit value: 0 or 1.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Bit {
    value: u8
}

#[derive(Debug)]
pub enum BitConstructionError {
    ValueOutOfRange,
}

impl Bit {
    pub const ZERO: Self = { Self { value: 0 } };
    pub const ONE: Self = { Self { value: 1 } };

    /// Creates new instance of `Bit` out of value.
    /// 
    /// # Errors
    /// Return `BitConstructionError::ValueOutOfRange` if passed value is
    /// greater than 1.
    pub fn new(value: u8) -> Result<Self, BitConstructionError> {
        if value > 1 {
            Err(BitConstructionError::ValueOutOfRange)
        }
        else
        {
            Ok(Self { value })
        }
    }

    /// Creates new instance of `Bit` out of value.
    /// 
    /// # Safety
    /// This is an unsafe operation and it is up to caller to ensure that
    /// `value` is either 0 or 1.
    pub unsafe fn new_unchecked(value: u8) -> Self {
        Self { value }
    }

    /// Returns numeric representation of current `Bit` value. Either
    /// 0 or 1.
    #[inline(always)]
    pub fn as_u8(&self) -> u8 { self.value }
}

impl BitAnd for Bit {
    type Output = Bit;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { Bit::new_unchecked(self.value & rhs.value) }
    }
}

impl BitOr for Bit {
    type Output = Bit;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { Bit::new_unchecked(self.value | rhs.value) }
    }
}

impl BitXor for Bit {
    type Output = Bit;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { Bit::new_unchecked(self.value ^ rhs.value) }
    }
}
