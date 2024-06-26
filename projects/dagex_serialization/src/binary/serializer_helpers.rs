#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation)]
use std::{
    hash::{Hash, BuildHasher},
    collections::HashMap,
    io::Write};

use dagex::{core::{ArrowDTO, DirectedGraphDTO}, phylo::PhylogeneticNetworkDTO};
use immutable_string::ImmutableString;

use crate::{Serializable, WriteError, WriteResult};

use super::BinarySerializer;

macro_rules! unsigned_serialization_fn {
    ( $numeric_type: ty ) => {
        impl<TWrite: Write> Serializable<BinarySerializer<TWrite>> for $numeric_type {
            fn serialize(&self, serializer: &mut BinarySerializer<TWrite>)
                -> Result<WriteResult<$numeric_type>, WriteError>
            {
                const SIZE: usize = core::mem::size_of::<$numeric_type>();
                const MAX_SIZE: usize = SIZE + (SIZE / 7) + 1;
                let mut buffer = [0u8; MAX_SIZE];
                let mut real_value = *self;
                let stream = serializer.stream_mut();
                if real_value == 0 {
                    buffer[0] = 1;
                    stream.write_all(&buffer[0..1])?;
                    return Ok(WriteResult::new(1usize));
                }
                
                let mut idx: usize = 0;
                loop {
                    let mut chunk = ((real_value & 0b01111111) as u8) << 1;
                    real_value >>= 7;
                    if real_value == 0 {
                        chunk |= 1;
                        buffer[idx] = chunk;
                        idx += 1;
                        break;
                    }
                    buffer[idx] = chunk;
                    idx += 1;
                }

                stream.write_all(&buffer[0..idx])?;
                Ok(WriteResult::new(idx))
            }
        }
    };
}

macro_rules! signed_serialization_fn {
    ( $numeric_type:ty, $from_type:ty ) => {
        impl<TWrite: Write> Serializable<BinarySerializer<TWrite>> for $numeric_type {
            fn serialize(&self, serializer: &mut BinarySerializer<TWrite>)
                -> Result<WriteResult<$numeric_type>, WriteError>
            {
                // NOTE: we are using zig-zag encoding for signed numbers.
                const SIZE: usize = <$numeric_type>::BITS as usize;
                let value = *self;
                let left = (value << 1) as $from_type;
                let right = (value >> (SIZE-1)) as $from_type;
                let result = (left ^ right).serialize(serializer)?;
                Ok(WriteResult::new(result.written_bytes()))
            }
        }
    };
}

unsigned_serialization_fn!(u32);
unsigned_serialization_fn!(u64);
unsigned_serialization_fn!(usize);
signed_serialization_fn!(i32, u32);
signed_serialization_fn!(i64, u64);
signed_serialization_fn!(isize, usize);

impl<TWrite: Write> Serializable<BinarySerializer<TWrite>> for ImmutableString {
    fn serialize(&self, serializer: &mut BinarySerializer<TWrite>)
        -> Result<WriteResult<ImmutableString>, WriteError>
    {
        let length = (self.len() as u32).serialize(serializer)?.written_bytes();
        let bytes = self.as_str().as_bytes();
        let stream = serializer.stream_mut();
        stream.write_all(bytes)?;
        Ok(WriteResult::new(length + bytes.len()))
    }
}

impl<TWrite: Write> Serializable<BinarySerializer<TWrite>> for ArrowDTO {
    fn serialize(&self, serializer: &mut BinarySerializer<TWrite>)
        -> Result<WriteResult<ArrowDTO>, WriteError>
    {
        let mut total = self.source().serialize(serializer)?.written_bytes();
        total += self.target().serialize(serializer)?.written_bytes();
        Ok(WriteResult::new(total))
    }
}

impl<TWrite, TItem> Serializable<BinarySerializer<TWrite>> for Vec<TItem>
    where TWrite: Write,
        TItem: Serializable<BinarySerializer<TWrite>>
{
    fn serialize(&self, serializer: &mut BinarySerializer<TWrite>)
        -> Result<WriteResult<Self>, WriteError>
    {
        let mut total = self.len().serialize(serializer)?.written_bytes();
        for item in self {
            total += item.serialize(serializer)?.written_bytes();
        }
        Ok(WriteResult::new(total))
    }
}

impl<TWrite, TKey, TValue, TBuildHasher> Serializable<BinarySerializer<TWrite>> for HashMap<TKey, TValue, TBuildHasher>
    where TWrite: Write,
        TKey: Serializable<BinarySerializer<TWrite>> + Ord + Hash + Eq,
        TValue: Serializable<BinarySerializer<TWrite>>,
        TBuildHasher: BuildHasher + Default
{
    fn serialize(&self, serializer: &mut BinarySerializer<TWrite>)
        -> Result<WriteResult<Self>, WriteError>
    {
        let mut total = self.len().serialize(serializer)?.written_bytes();
        let mut sorted = self.iter().collect::<Vec<_>>();
        sorted.sort_by_key(|x| x.0);
        for kvp in sorted {
            total += kvp.0.serialize(serializer)?.written_bytes();
            total += kvp.1.serialize(serializer)?.written_bytes();
        }
        Ok(WriteResult::new(total))
    }
}


impl<TWrite: Write> Serializable<BinarySerializer<TWrite>> for DirectedGraphDTO {
    fn serialize(&self, serializer: &mut BinarySerializer<TWrite>)
        -> Result<WriteResult<DirectedGraphDTO>, WriteError>
    {
        const ZERO: usize = 0;
        let mut total = self.number_of_nodes().serialize(serializer)?.written_bytes();
        total += self.arrows().serialize(serializer)?.written_bytes();
        total += ZERO.serialize(serializer)?.written_bytes();
        Ok(WriteResult::new(total))
    }
}


impl<TWrite: Write> Serializable<BinarySerializer<TWrite>> for PhylogeneticNetworkDTO {
    fn serialize(&self, serializer: &mut BinarySerializer<TWrite>)
        -> Result<WriteResult<PhylogeneticNetworkDTO>, WriteError>
    {
        let dg = self.graph();
        let mut total = dg.number_of_nodes().serialize(serializer)?.written_bytes();
        total += dg.arrows().serialize(serializer)?.written_bytes();
        total += self.taxa().serialize(serializer)?.written_bytes();
        Ok(WriteResult::new(total))
    }
}
