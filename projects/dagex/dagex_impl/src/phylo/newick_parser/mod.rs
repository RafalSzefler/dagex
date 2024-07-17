use std::io::Read;

mod error;
mod ok;
mod context;

use context::NewickParseContext;
pub use error::*;
pub use ok::*;

use raf_newick::deserializer::deserialize;

/// Parses Newick formatted stream into [`PhylogeneticNetwork`].
/// 
/// # Errors
/// * [`NewickParseError::ContentError`] if invalid graph
/// * [`NewickParseError::InputError`] forwarded from underlying stream
/// * [`NewickParseError::Utf8`] if content is not a valid UTF-8 string
pub fn parse_newick<TRead: Read>(input: &mut TRead)
    -> Result<NewickParseOk, NewickParseError>
{
    let deserialize_ok = deserialize(input)?;
    let graph = &deserialize_ok.graph;
    let ctx = NewickParseContext::new(graph);
    let network = ctx.parse()?;
    Ok(NewickParseOk {
        network: network,
        read_bytes: deserialize_ok.read_bytes,
    })
}

/// Parses Newick formatted `&str` into [`PhylogeneticNetwork`].
/// 
/// # Errors
/// * [`NewickParseError::ContentError`] if invalid graph
/// * [`NewickParseError::InputError`] forwarded from underlying stream
/// * [`NewickParseError::Utf8`] if content is not a valid UTF-8 string
#[inline(always)]
pub fn parse_newick_from_str(input: &str)
    -> Result<NewickParseOk, NewickParseError>
{
    let mut stream = input.as_bytes();
    parse_newick(&mut stream)
}
