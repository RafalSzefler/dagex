use std::io::Read;

use crate::{binary_deserializer::deserializer_helpers::{deserialize_arrow, deserialize_dg, deserialize_pn}, traits_deserializer::ReadResult, Deserializer, ReadError, TypeInfo, WithTypeInfo};

use super::deserializer_helpers::{
    deserialize_i32,
    deserialize_i64,
    deserialize_imm,
    deserialize_isize,
    deserialize_u32,
    deserialize_u64,
    deserialize_usize};

pub struct BinaryDeserializer<TRead: Read> {
    stream: TRead,
}

impl<TRead: Read> Deserializer<TRead> for BinaryDeserializer<TRead> {
    fn from_stream(stream: TRead) -> Self {
        Self { stream }
    }

    fn release(self) -> TRead {
        self.stream
    }

    fn read<T>(&mut self) -> Result<ReadResult<T>, ReadError>
        where T: WithTypeInfo
    {
        macro_rules! mutate {
            ( $e:expr ) => {
                {
                    let mut read_result = { $e }?.release();
                    let read_bytes = read_result.read_bytes;
                    let item = core::mem::take(&mut read_result.item);
                    let src = core::ptr::from_ref(&item).cast::<T>();
                    let dst = unsafe { core::ptr::read(src) };

                    #[allow(forgetting_copy_types, clippy::forget_non_drop)]
                    {
                        std::mem::forget(item);
                    }

                    return Ok(ReadResult::new(dst, read_bytes));
                }
            };
        }


        match T::type_info() {
            TypeInfo::I32 => mutate!(deserialize_i32(&mut self.stream)),
            TypeInfo::U32 => mutate!(deserialize_u32(&mut self.stream)),
            TypeInfo::I64 => mutate!(deserialize_i64(&mut self.stream)),
            TypeInfo::U64 => mutate!(deserialize_u64(&mut self.stream)),
            TypeInfo::Usize => mutate!(deserialize_usize(&mut self.stream)),
            TypeInfo::Isize => mutate!(deserialize_isize(&mut self.stream)),
            TypeInfo::ImmutableString => mutate!(deserialize_imm(&mut self.stream)),
            TypeInfo::ArrowDTO => mutate!(deserialize_arrow(&mut self.stream)),
            TypeInfo::DirectedGraphDTO => mutate!(deserialize_dg(&mut self.stream)),
            TypeInfo::PhylogeneticNetworkDTO => mutate!(deserialize_pn(&mut self.stream)),
        }
    }
}
