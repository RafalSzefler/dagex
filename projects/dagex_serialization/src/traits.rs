use dagex::{core::{ArrowDTO, DirectedGraphDTO}, phylo::PhylogeneticNetworkDTO};
use immutable_string::ImmutableString;

pub enum TypeInfo {
    I32,
    U32,
    I64,
    U64,
    Usize,
    Isize,
    ImmutableString,
    ArrowDTO,
    DirectedGraphDTO,
    PhylogeneticNetworkDTO,
}

pub trait WithTypeInfo {
    fn type_info() -> TypeInfo;
}

impl WithTypeInfo for i32 {
    #[inline(always)]
    fn type_info() -> TypeInfo { TypeInfo::I32 }
}

impl WithTypeInfo for u32 {
    #[inline(always)]
    fn type_info() -> TypeInfo { TypeInfo::U32 }
}

impl WithTypeInfo for i64 {
    #[inline(always)]
    fn type_info() -> TypeInfo { TypeInfo::I64 }
}

impl WithTypeInfo for u64 {
    #[inline(always)]
    fn type_info() -> TypeInfo { TypeInfo::U64 }
}

impl WithTypeInfo for usize {
    #[inline(always)]
    fn type_info() -> TypeInfo { TypeInfo::Usize }
}

impl WithTypeInfo for isize {
    #[inline(always)]
    fn type_info() -> TypeInfo { TypeInfo::Isize }
}

impl WithTypeInfo for ImmutableString {
    #[inline(always)]
    fn type_info() -> TypeInfo { TypeInfo::ImmutableString }
}

impl WithTypeInfo for ArrowDTO {
    #[inline(always)]
    fn type_info() -> TypeInfo { TypeInfo::ArrowDTO }
}

impl WithTypeInfo for DirectedGraphDTO {
    #[inline(always)]
    fn type_info() -> TypeInfo { TypeInfo::DirectedGraphDTO }
}

impl WithTypeInfo for PhylogeneticNetworkDTO {
    #[inline(always)]
    fn type_info() -> TypeInfo { TypeInfo::PhylogeneticNetworkDTO }
}
