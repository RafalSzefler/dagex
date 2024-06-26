use std::io::Write;

use crate::{
    binary_serializer::serializer_helpers::{
        serialize_i32,
        serialize_i64,
        serialize_u32,
        serialize_u64,
        serialize_arrow,
        serialize_dg,
        serialize_imm,
        serialize_isize,
        serialize_pn,
        serialize_usize
    },
    traits_serializer::WriteResult,
    Serializer,
    TypeInfo,
    WithTypeInfo,
    WriteError};

pub struct BinarySerializer<TWrite: Write> {
    stream: TWrite,
}

impl<TWrite: Write> Serializer<TWrite> for BinarySerializer<TWrite> {
    fn from_stream(stream: TWrite) -> Self {
        Self { stream }
    }

    fn release(self) -> TWrite {
        self.stream
    }

    fn write<T>(&mut self, item: &T)-> Result<WriteResult<T>, WriteError>
        where T: WithTypeInfo
    {
        macro_rules! cast {
            ( $e: expr ) => {
                {
                    let ptr = core::ptr::from_ref($e).cast();
                    unsafe { &*ptr }
                }
            };
        }
        
        macro_rules! as_num {
            ( $e:expr ) => {
                {
                    let ptr = core::ptr::from_ref($e);
                    unsafe { *(ptr.cast::<()>().cast()) }
                }
            }
        }

        let written_bytes = match T::type_info() {
            TypeInfo::I32 => serialize_i32(&mut self.stream, as_num!(item)),
            TypeInfo::U32 => serialize_u32(&mut self.stream, as_num!(item)),
            TypeInfo::I64 => serialize_i64(&mut self.stream, as_num!(item)),
            TypeInfo::U64 => serialize_u64(&mut self.stream, as_num!(item)),
            TypeInfo::Isize => serialize_isize(&mut self.stream, as_num!(item)),
            TypeInfo::Usize => serialize_usize(&mut self.stream, as_num!(item)),
            TypeInfo::ImmutableString => serialize_imm(&mut self.stream, cast!(item)),
            TypeInfo::ArrowDTO => serialize_arrow(&mut self.stream, cast!(item)),
            TypeInfo::DirectedGraphDTO => serialize_dg(&mut self.stream, cast!(item)),
            TypeInfo::PhylogeneticNetworkDTO => serialize_pn(&mut self.stream, cast!(item)),
        }?;

        Ok(WriteResult::new(written_bytes))
    }
}
