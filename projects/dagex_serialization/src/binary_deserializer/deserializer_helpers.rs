#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation)]

use std::io::Read;

use array::Array;
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
                let mut total_size: usize = 0;
                let buffer = &mut [0u8];

                loop {
                    total_size += 1;
                    stream.read_exact(buffer)?;
                    let value = buffer[0] >> 1;
                    result = result.checked_shl(7).ok_or_else(overflow_to_error)?;
                    result = result.checked_add(value as $numeric_type).ok_or_else(overflow_to_error)?;
                    if (value & 0b1) == 0b0 {
                        break;
                    }
                }

                Ok(ReadResult::new(result, total_size))
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
                let size = result.read_bytes();
                let value = result.release();
                let left = (value >> 1) as $numeric_type;
                let right = -((value & 1) as $numeric_type);
                Ok(ReadResult::new(left ^ right, size))
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

    let read_result = deserialize_usize(stream)?;
    let read_size = read_result.read_bytes();
    let imm_len = read_result.release();

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

    match core::str::from_utf8(&buffer) {
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
