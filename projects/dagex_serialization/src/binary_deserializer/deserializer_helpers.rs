#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::cast_possible_wrap)]

use std::{collections::HashMap, io::Read};

use array::Array;
use dagex::{core::{ArrowDTO, DirectedGraphDTO}, phylo::PhylogeneticNetworkDTO};
use immutable_string::ImmutableString;

use crate::{ReadResult, ReadError};

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
    ( $numeric_type:ident ) => {
        paste::item! {
            #[allow(dead_code)]
            pub(super) fn [< deserialize_ $numeric_type >]<TRead: Read>(stream: &mut TRead)
                -> Result<ReadResult<$numeric_type>, ReadError>
            {
                let mut result: $numeric_type = 0;
                let mut total_size: u32 = 0;
                let mut buffer = [0u8; 1];

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
    ( $numeric_type:ident, $from_type: ident ) => {
        paste::item! {
            #[allow(dead_code)]
            pub(super) fn [< deserialize_ $numeric_type >]<TRead: Read>(stream: &mut TRead)
                -> Result<ReadResult<$numeric_type>, ReadError>
            {
                // NOTE: we are using zig-zag decoding for signed numbers.
                let result = [< deserialize_ $from_type >]::<TRead>(stream)?;
                let owned = result.release();
                let value = owned.item;
                let left = (value >> 1) as $numeric_type;
                let right = -((value & 1) as $numeric_type);
                Ok(ReadResult::new(left ^ right, owned.read_bytes))
            }
        }
    };
}

unsigned_deserialization_fn!(u32);
unsigned_deserialization_fn!(u64);
unsigned_deserialization_fn!(usize);
signed_deserialization_fn!(i32, u32);
signed_deserialization_fn!(i64, u64);
signed_deserialization_fn!(isize, usize);

pub(super) fn deserialize_imm<TRead: Read>(stream: &mut TRead)
    -> Result<ReadResult<ImmutableString>, ReadError>
{
    const MAX_INLINE_SIZE: usize = 128;

    let read_result = deserialize_usize(stream)?.release();
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


pub(super) fn deserialize_arrow<TRead: Read>(stream: &mut TRead)
    -> Result<ReadResult<ArrowDTO>, ReadError>
{
    let src_result = deserialize_i32(stream)?.release();
    let dst_result = deserialize_i32(stream)?.release();
    let item = ArrowDTO::new(src_result.item, dst_result.item);
    Ok(ReadResult::new(item, src_result.read_bytes + dst_result.read_bytes))
}

pub(super) fn deserialize_hash_map<TRead: Read>(stream: &mut TRead)
    -> Result<ReadResult<HashMap<i32, ImmutableString>>, ReadError>
{
    let mut total_size: usize = 0;
    let size_result = deserialize_usize(stream)?.release();
    total_size += size_result.read_bytes;
    let items_count = size_result.item;
    let mut map = HashMap::<i32, ImmutableString>::with_capacity(items_count);
    if items_count == 0 {
        return Ok(ReadResult::new(map, total_size));
    }

    for _ in 0..items_count {
        let key = deserialize_i32(stream)?.release();
        let value = deserialize_imm(stream)?.release();
        map.insert(key.item, value.item);
        total_size += key.read_bytes + value.read_bytes;
    }

    Ok(ReadResult::new(map, total_size))
}


pub(super) fn deserialize_dg<TRead: Read>(stream: &mut TRead)
    -> Result<ReadResult<DirectedGraphDTO>, ReadError>
{
    let mut total_size: usize = 0;
    let number_of_nodes = deserialize_i32(stream)?.release();
    total_size += number_of_nodes.read_bytes;
    let arrows_count = deserialize_usize(stream)?.release();
    total_size += arrows_count.read_bytes;
    let mut arrows_vec = Vec::<ArrowDTO>::with_capacity(arrows_count.item);
    for _ in 0..arrows_count.item {
        let arrow_result = deserialize_arrow(stream)?.release();
        total_size += arrow_result.read_bytes;
        arrows_vec.push(arrow_result.item);
    }

    total_size += deserialize_hash_map(stream)?.release().read_bytes;
    let dg = DirectedGraphDTO::new(number_of_nodes.item, arrows_vec);
    Ok(ReadResult::new(dg, total_size))
}

pub(super) fn deserialize_pn<TRead: Read>(stream: &mut TRead)
    -> Result<ReadResult<PhylogeneticNetworkDTO>, ReadError>
{
    let mut total_size: usize = 0;
    let number_of_nodes = deserialize_i32(stream)?.release();
    total_size += number_of_nodes.read_bytes;
    let arrows_count = deserialize_usize(stream)?.release();
    total_size += arrows_count.read_bytes;
    let mut arrows_vec = Vec::<ArrowDTO>::with_capacity(arrows_count.item);
    for _ in 0..arrows_count.item {
        let arrow_result = deserialize_arrow(stream)?.release();
        total_size += arrow_result.read_bytes;
        arrows_vec.push(arrow_result.item);
    }

    let map = deserialize_hash_map(stream)?.release();
    total_size += map.read_bytes;
    let dg = DirectedGraphDTO::new(number_of_nodes.item, arrows_vec);
    let pn = PhylogeneticNetworkDTO::new(dg, map.item);
    Ok(ReadResult::new(pn, total_size))
}
