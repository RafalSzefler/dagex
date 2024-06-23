pub(crate) trait Conv<T> {
    fn convert(&self) -> T;
}

impl Conv<usize> for u32 {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss)]
    #[inline(always)]
    fn convert(&self) -> usize { *self as usize }
}

impl Conv<usize> for i32 {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss)]
    #[inline(always)]
    fn convert(&self) -> usize { *self as usize }
}

impl Conv<usize> for usize {
    #[inline(always)]
    fn convert(&self) -> usize { *self }
}
