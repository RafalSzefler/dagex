use std::io::Read;

use crate::{traits_deserializer::ReadResult, Deserializer, ReadError, TypeInfo, WithTypeInfo};

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
                    let read_result = { $e }?;
                    let read_bytes = read_result.read_bytes();
                    let src = read_result.release();
                    let dst: T = unsafe { core::mem::transmute_copy(&src) };
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
            TypeInfo::ArrowDTO => todo!(),
            TypeInfo::DirectedGraphDTO => todo!(),
            TypeInfo::PhylogeneticNetworkDTO => todo!(),
        }
    }
}
