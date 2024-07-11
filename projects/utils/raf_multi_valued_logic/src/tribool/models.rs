#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct TriBool {
    value: u8,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NotBoolError;

impl TriBool {
    pub const FALSE: TriBool = TriBool::internal_new(0);
    pub const UNKNOWN: TriBool = TriBool::internal_new(1);
    pub const TRUE: TriBool = TriBool::internal_new(2);

    #[inline(always)]
    const fn internal_new(value: u8) -> Self { Self { value } }

    /// Creates new [`TriBool`] out of `u8` value. There are only
    /// three valid values: 0 (FALSE), 1 (UNKNOWN) and 2 (TRUE).
    /// 
    /// # Safety
    /// The behaviour is undefined if called with `value > 2`. 
    #[inline(always)]
    pub unsafe fn new_unchecked(value: u8) -> Self {
        Self::internal_new(value)
    }

    #[must_use]
    pub const fn and(self, rhs: Self) -> Self {
        const TABLE: [[TriBool; 3]; 3] = [
            [TriBool::FALSE,  TriBool::FALSE,    TriBool::FALSE],
            [TriBool::FALSE,  TriBool::UNKNOWN,  TriBool::UNKNOWN],
            [TriBool::FALSE,  TriBool::UNKNOWN,  TriBool::TRUE],
        ];
        TABLE[self.value as usize][rhs.value as usize]
    }

    #[must_use]
    pub const fn or(self, rhs: Self) -> Self {
        const TABLE: [[TriBool; 3]; 3] = [
            [TriBool::FALSE,    TriBool::UNKNOWN,  TriBool::TRUE],
            [TriBool::UNKNOWN,  TriBool::UNKNOWN,  TriBool::TRUE],
            [TriBool::TRUE,     TriBool::TRUE,     TriBool::TRUE],
        ];
        TABLE[self.value as usize][rhs.value as usize]
    }

    /// Converts `TriBool::TRUE` to `TriBool::FALSE` and vice versa.
    /// Does not affect `TriBool::UNKNOWN`.
    #[must_use]
    #[inline(always)]
    pub const fn neg(self) -> Self {
        Self::internal_new(2 - self.value)
    }

    /// Efficiently encodes fact that `self` is `TriBool::TRUE`.
    #[must_use]
    #[inline(always)]
    pub const fn is_certain(self) -> TriBool {
        TriBool::internal_new((self.value / 2) * 2)
    }

    /// Efficiently encodes fact that `self` is either
    /// `TriBool::TRUE` or `TriBool::UNKNOWN`.
    #[must_use]
    #[inline(always)]
    pub const fn is_possible(self) -> TriBool {
        TriBool::internal_new(((self.value + 1) / 2) * 2)
    }

    /// Converts [`TriBool`] into corresponding [`bool`] if `self` is
    /// [`TriBool::TRUE`] or [`TriBool::FALSE`].
    /// 
    /// # Errors
    /// * [`NotBoolError`] if `self` is [`TriBool::UNKNOWN`].
    #[inline(always)]
    pub const fn as_bool(self) -> Result<bool, NotBoolError> {
        if self.value == 1 {
            return Err(NotBoolError);
        }

        Ok(self.value == 2)
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(super) const fn as_u8(self) -> u8 {
        self.value
    }

    /// Returns `self` as `&'static str`.
    /// 
    /// # Panics
    /// When `self` was not safely constructed and doesn't
    /// represent valid [`TriBool`] instance.
    pub fn as_str(self) -> &'static str {
        match self.value {
            0 => "FALSE",
            1 => "UNKNOWN",
            2 => "TRUE",
            _ => panic!("Invalid TriBool instance.")
        }
    }
}

impl core::fmt::Debug for TriBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TriBool")
            .field("value", &self.as_str())
            .finish()
    }
}

impl core::fmt::Display for TriBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<bool> for TriBool {
    fn from(value: bool) -> Self {
        if value {
            TriBool::TRUE
        } else {
            TriBool::FALSE
        }
    }
}
