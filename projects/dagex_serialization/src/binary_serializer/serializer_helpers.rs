#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation)]
use std::{cmp::Ordering, io::Write};

use dagex::{core::{ArrowDTO, DirectedGraphDTO}, phylo::PhylogeneticNetworkDTO};
use immutable_string::ImmutableString;
use itertools::Itertools;

use crate::WriteError;

macro_rules! unsigned_serialization_fn {
    ( $numeric_type:ident ) => {
        paste::item! {
            pub(super) fn [< serialize_ $numeric_type >]<TWrite: Write>(stream: &mut TWrite, value: $numeric_type)
                -> Result<usize, WriteError>
            {
                const SIZE: usize = core::mem::size_of::<$numeric_type>();
                const MAX_SIZE: usize = SIZE + (SIZE / 7) + 1;
                let mut buffer = [0u8; MAX_SIZE];
                let mut real_value = value;
                if real_value == 0 {
                    buffer[0] = 1;
                    stream.write_all(&buffer[0..1])?;
                    return Ok(1usize);
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
                Ok(idx)
            }
        }
    };
}

macro_rules! signed_serialization_fn {
    ( $numeric_type:ident, $from_type:ident ) => {
        paste::item! {
            #[allow(dead_code)]
            pub(super) fn [< serialize_ $numeric_type >]<TWrite: Write>(stream: &mut TWrite, value: $numeric_type)
                -> Result<usize, WriteError>
            {
                // NOTE: we are using zig-zag encoding for signed numbers.
                const SIZE: usize = $numeric_type::BITS as usize;
                let left = (value << 1) as $from_type;
                let right = (value >> (SIZE-1)) as $from_type;
                [< serialize_ $from_type >](stream, left ^ right)
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

pub(super) fn serialize_imm<TWrite: Write>(stream: &mut TWrite, value: &ImmutableString)
    -> Result<usize, WriteError>
{
    let length = serialize_u32(stream, value.len() as u32)?;
    let bytes = value.as_str().as_bytes();
    stream.write_all(bytes)?;
    Ok(length + bytes.len())
}

pub(super) fn serialize_arrow<TWrite: Write>(stream: &mut TWrite, value: &ArrowDTO)
    -> Result<usize, WriteError>
{
    let mut total = serialize_i32(stream, value.source())?;
    total += serialize_i32(stream, value.target())?;
    Ok(total)
}

pub(super) fn serialize_dg<TWrite: Write>(stream: &mut TWrite, value: &DirectedGraphDTO)
    -> Result<usize, WriteError>
{
    let mut total = serialize_i32(stream, value.number_of_nodes())?;
    for arr in value.arrows() {
        total += serialize_arrow(stream, arr)?;
    }
    total += serialize_usize(stream, 0)?;
    Ok(total)
}

struct NodeImmPair<'a> {
    pub node: i32,
    pub imm: &'a ImmutableString,
}

impl<'a> NodeImmPair<'a> {
    #[inline(always)]
    fn compare(&self, other: &Self) -> Ordering {
        self.node.cmp(&other.node)
    }
}

impl<'a> PartialEq for NodeImmPair<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == Ordering::Equal
    }
}

impl<'a> Eq for NodeImmPair<'a> { }

impl<'a> PartialOrd for NodeImmPair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for NodeImmPair<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.compare(other)
    }
}

pub(super) fn serialize_pn<TWrite: Write>(stream: &mut TWrite, value: &PhylogeneticNetworkDTO)
    -> Result<usize, WriteError>
{
    let dg = value.graph();
    let mut total = serialize_i32(stream, dg.number_of_nodes())?;
    for arr in dg.arrows() {
        total += serialize_arrow(stream, arr)?;
    }

    let taxa = value.taxa();
    total += serialize_usize(stream, taxa.len())?;
    let iterator = taxa.iter()
        .map(|p| { NodeImmPair { node: *(p.0), imm: p.1 }})
        .sorted();

    for kvp in iterator {
        total += serialize_i32(stream, kvp.node)?;
        total += serialize_imm(stream, kvp.imm)?;
    }

    Ok(total)
}
