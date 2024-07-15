use serde::{de::Visitor, ser::SerializeTuple, Deserialize, Serialize};

use crate::core::ArrowDTO;

const STRUCT_NAME: &str = "ArrowDTO";
const SOURCE_FIELD: &str = "source";
const TARGET_FIELD: &str = "target";

impl Serialize for ArrowDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let mut state = serializer.serialize_tuple(2)?;
        state.serialize_element(&self.source())?;
        state.serialize_element(&self.target())?;
        state.end()
    }
}

struct ArrowDTOVisitor;

impl<'de> Visitor<'de> for ArrowDTOVisitor {
    type Value = ArrowDTO;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(STRUCT_NAME)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
    {
        let source = seq.next_element()?.unwrap();
        let target = seq.next_element()?.unwrap();
        Ok(ArrowDTO::new(source, target))
    }
}

impl<'de> Deserialize<'de> for ArrowDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_struct(STRUCT_NAME, &[SOURCE_FIELD, TARGET_FIELD], ArrowDTOVisitor)
    }
}
