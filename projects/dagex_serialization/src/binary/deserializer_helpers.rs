#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::cast_possible_wrap)]

use std::{
    collections::HashMap, hash::{BuildHasher, Hash}, io::Read};

use array::Array;
use dagex::{core::{ArrowDTO, DirectedGraphDTO}, phylo::PhylogeneticNetworkDTO};
use immutable_string::ImmutableString;

use crate::{Deserializable, ReadError, ReadResult};

use super::BinaryDeserializer;

#[inline(always)]
fn overflow_to_error() -> ReadError {
    ReadError::InvalidContent("Unsigned overflow detected.".to_owned())
}

#[inline(always)]
fn notutf8_to_error() -> ReadError {
    ReadError::InvalidContent("Embedded string is not utf-8.".to_owned())
}

#[inline(always)]
fn invalid_imm_to_error() -> ReadError {
    ReadError::InvalidContent("Couldn't construct ImmutableString for embedded string.".to_owned())
}


macro_rules! unsigned_deserialization_fn {
    ( $numeric_type: ty ) => {
        impl<TRead: Read> Deserializable<BinaryDeserializer<TRead>> for $numeric_type {
            fn deserialize(deserializer: &mut BinaryDeserializer<TRead>)
                -> Result<ReadResult<Self>, ReadError>
            {
                let mut result: $numeric_type = 0;
                let mut total_size: u32 = 0;
                let mut buffer = [0u8; 1];
                let stream = deserializer.stream_mut();
        
                loop {
                    stream.read_exact(&mut buffer)?;
                    let byte: u8 = buffer[0];
                    let value = {
                        let initial = (byte >> 1) as $numeric_type;
                        initial.checked_shl(7*total_size)
                    }.ok_or_else(overflow_to_error)?;
        
                    result |= value; 
                    total_size += 1;
                    if (byte & 1u8) == 1u8 {
                        break;
                    }
                }
        
                Ok(ReadResult::new(result, total_size as usize))
            }
        }
    };
}

macro_rules! signed_deserialization_fn {
    ( $numeric_type: ty, $from_type: ty ) => {
        impl<TRead: Read> Deserializable<BinaryDeserializer<TRead>> for $numeric_type {
            fn deserialize(deserializer: &mut BinaryDeserializer<TRead>)
                -> Result<ReadResult<Self>, ReadError>
            {
                // NOTE: we are using zig-zag decoding for signed numbers.
                let result = <$from_type>::deserialize(deserializer)?;
                let owned = result.release();
                let value = owned.item;
                let left = (value >> 1) as $numeric_type;
                let right = -((value & 1) as $numeric_type);
                Ok(ReadResult::new(left ^ right, owned.read_bytes))
            }
        }
    }
}

unsigned_deserialization_fn!(u32);
unsigned_deserialization_fn!(u64);
unsigned_deserialization_fn!(usize);
signed_deserialization_fn!(i32, u32);
signed_deserialization_fn!(i64, u64);
signed_deserialization_fn!(isize, usize);

impl<TRead: Read> Deserializable<BinaryDeserializer<TRead>> for ImmutableString {
    fn deserialize(deserializer: &mut BinaryDeserializer<TRead>)
        -> Result<ReadResult<Self>, ReadError>
    {
        const MAX_INLINE_SIZE: usize = 128;

        let read_result = usize::deserialize(deserializer)?.release();
        let read_size = read_result.read_bytes;
        let imm_len = read_result.item;

        let mut inline_buffer;
        let mut array;

        let buffer = if imm_len < MAX_INLINE_SIZE {
            inline_buffer = [0u8; MAX_INLINE_SIZE];
            &mut inline_buffer[0..imm_len]
        }
        else
        {
            array = Array::new(imm_len);
            array.as_slice_mut()
        };

        let stream = deserializer.stream_mut();
        stream.read_exact(buffer)?;
        let imm: ImmutableString;

        match core::str::from_utf8(buffer) {
            Ok(text) => {
                match ImmutableString::get(text) {
                    Ok(value) => {
                        imm = value;
                    },
                    Err(_) => {
                        return Err(invalid_imm_to_error());
                    },
                }
            },
            Err(_) => {
                return Err(notutf8_to_error());
            },
        }

        Ok(ReadResult::new(imm, read_size + imm_len))
    }
}

impl<TRead: Read> Deserializable<BinaryDeserializer<TRead>> for ArrowDTO {
    fn deserialize(deserializer: &mut BinaryDeserializer<TRead>)
        -> Result<ReadResult<Self>, ReadError>
    {
        let src_result = i32::deserialize(deserializer)?.release();
        let dst_result = i32::deserialize(deserializer)?.release();
        let item = ArrowDTO::new(src_result.item, dst_result.item);
        Ok(ReadResult::new(item, src_result.read_bytes + dst_result.read_bytes))
    }
}

impl<TRead, TItem> Deserializable<BinaryDeserializer<TRead>> for Vec<TItem>
    where TRead: Read,
        TItem: Deserializable<BinaryDeserializer<TRead>>
{
    fn deserialize(deserializer: &mut BinaryDeserializer<TRead>)
        -> Result<ReadResult<Self>, ReadError>
    {
        let mut total_size = 0usize;
        let read_result = usize::deserialize(deserializer)?.release();
        total_size += read_result.read_bytes;
        let mut final_item = Vec::with_capacity(read_result.item);
        for _ in 0..read_result.item {
            let item = TItem::deserialize(deserializer)?.release();
            total_size += item.read_bytes;
            final_item.push(item.item);
        }
        Ok(ReadResult::new(final_item, total_size))
    }
}

impl<TRead, TKey, TValue, TBuildHasher> Deserializable<BinaryDeserializer<TRead>> for HashMap<TKey, TValue, TBuildHasher>
    where TRead: Read,
        TKey: Deserializable<BinaryDeserializer<TRead>> + Ord + Hash + Eq,
        TValue: Deserializable<BinaryDeserializer<TRead>>,
        TBuildHasher: BuildHasher + Default
{
    fn deserialize(deserializer: &mut BinaryDeserializer<TRead>)
        -> Result<ReadResult<Self>, ReadError>
    {
        let mut total_size: usize = 0;
        let size_result = usize::deserialize(deserializer)?.release();
        total_size += size_result.read_bytes;
        let items_count = size_result.item;
        let mut map = HashMap::<TKey, TValue, TBuildHasher>::with_hasher(TBuildHasher::default());
        if items_count == 0 {
            return Ok(ReadResult::new(map, total_size));
        }

        for _ in 0..items_count {
            let key = TKey::deserialize(deserializer)?.release();
            let value = TValue::deserialize(deserializer)?.release();
            map.insert(key.item, value.item);
            total_size += key.read_bytes + value.read_bytes;
        }

        Ok(ReadResult::new(map, total_size))
    }
}

impl<TRead: Read> Deserializable<BinaryDeserializer<TRead>> for DirectedGraphDTO {
    fn deserialize(deserializer: &mut BinaryDeserializer<TRead>)
        -> Result<ReadResult<Self>, ReadError>
    {
        let mut total_size = 0usize;
        let number_of_nodes = i32::deserialize(deserializer)?.release();
        total_size += number_of_nodes.read_bytes;

        let arrows = Vec::<ArrowDTO>::deserialize(deserializer)?.release();
        total_size += arrows.read_bytes;
        total_size += HashMap::<i32, ImmutableString>::deserialize(deserializer)?.release().read_bytes;
        let dg = DirectedGraphDTO::new(number_of_nodes.item, arrows.item);
        Ok(ReadResult::new(dg, total_size))
    }
}

impl<TRead: Read> Deserializable<BinaryDeserializer<TRead>> for PhylogeneticNetworkDTO {
    fn deserialize(deserializer: &mut BinaryDeserializer<TRead>)
        -> Result<ReadResult<Self>, ReadError>
    {
        let mut total_size = 0usize;
        let number_of_nodes = i32::deserialize(deserializer)?.release();
        total_size += number_of_nodes.read_bytes;

        let arrows = Vec::<ArrowDTO>::deserialize(deserializer)?.release();
        total_size += arrows.read_bytes;
        
        let taxa = HashMap::<i32, ImmutableString>::deserialize(deserializer)?.release();
        total_size += taxa.read_bytes;

        let dg = DirectedGraphDTO::new(number_of_nodes.item, arrows.item);
        let pn = PhylogeneticNetworkDTO::new(dg, taxa.item);
        Ok(ReadResult::new(pn, total_size))
    }
}
