use std::{io::Read, marker::PhantomData};

use raf_newick::{ast::NewickGraph, deserializer::{deserialize, DeserializeError}};

use crate::phylo::PhylogeneticNetwork;

pub struct NewickParser {
    _phantom: PhantomData<()>,
}

#[derive(Debug)]
pub enum NewickParseError {
    ContentError(String),
    InputError(std::io::Error),
    Utf8(std::str::Utf8Error),
}

impl From<DeserializeError> for NewickParseError {
    fn from(value: DeserializeError) -> Self {
        match value {
            DeserializeError::FormatError(text)
                => Self::ContentError(text),
            DeserializeError::GraphError(err) => {
                let msg = format!("Invalid graph: {err:?}");
                Self::ContentError(msg)
            },
            DeserializeError::InputError(err) => Self::InputError(err),
            DeserializeError::Utf8(err) => Self::Utf8(err),
        }
    }
}

pub struct NewickParseOk {
    pub network: PhylogeneticNetwork,
    pub read_bytes: usize,
}

impl NewickParser {
    /// Parses [`PhylogeneticNetwork`] out of input stream.
    /// 
    /// # Errors
    /// * [`NewickParseError::ContentError`] if invalid graph
    /// * [`NewickParseError::InputError`] forwarded from underlying stream
    /// * [`NewickParseError::Utf8`] if content is not a valid UTF-8 string
    pub fn parse<TRead: Read>(&mut self, input: &mut TRead)
        -> Result<NewickParseOk, NewickParseError>
    {
        let deserialize_ok = deserialize(input)?;
        let network = self.convert(&deserialize_ok.graph)?;
        Ok(NewickParseOk {
            network: network,
            read_bytes: deserialize_ok.read_bytes,
        })
    }

    fn convert(&mut self, newick_graph: &NewickGraph)
        -> Result<PhylogeneticNetwork, NewickParseError>
    {
        let _ = newick_graph;
        todo!()
    }
}

impl Default for NewickParser {
    fn default() -> Self {
        Self { _phantom: PhantomData }
    }
}
