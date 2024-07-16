use raf_newick::deserializer::DeserializeError;

use crate::phylo::PhylogeneticNetworkFromError;

#[derive(Debug)]
pub enum NewickParseError {
    ContentError(String),
    InputError(std::io::Error),
    Utf8(std::str::Utf8Error),
    PhylogeneticNetworkError(PhylogeneticNetworkFromError),
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

impl From<PhylogeneticNetworkFromError> for NewickParseError {
    fn from(value: PhylogeneticNetworkFromError) -> Self {
        Self::PhylogeneticNetworkError(value)
    }
}
