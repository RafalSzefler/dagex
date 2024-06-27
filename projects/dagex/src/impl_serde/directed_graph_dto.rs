use serde::{de::{self, Visitor}, ser::SerializeStruct, Deserialize, Serialize};

use crate::core::DirectedGraphDTO;

const STRUCT_NAME: &str = "DirectedGraphDTO";
const NODES_LEN_FIELD: &str = "number_of_nodes";
const ARROWS_FIELD: &str = "arrows";

impl Serialize for DirectedGraphDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let mut state = serializer.serialize_struct(STRUCT_NAME, 2)?;
        state.serialize_field(NODES_LEN_FIELD, &self.number_of_nodes())?;
        state.serialize_field(ARROWS_FIELD, &self.arrows())?;
        state.end()
    }
}

struct DirectedGraphDTOVisitor;

impl<'de> Visitor<'de> for DirectedGraphDTOVisitor {
    type Value = DirectedGraphDTO;

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
        Ok(DirectedGraphDTO::new(source, target))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
    {
        let mut source = None;
        let mut target = None;
        while let Some(key) = map.next_key()? {
            match key {
                NODES_LEN_FIELD => {
                    if source.is_some() {
                        return Err(de::Error::duplicate_field(NODES_LEN_FIELD));
                    }
                    source = Some(map.next_value()?);
                },
                ARROWS_FIELD => {
                    if target.is_some() {
                        return Err(de::Error::duplicate_field(ARROWS_FIELD));
                    }
                    target = Some(map.next_value()?);
                },
                _ => { }
            }
        }

        let source = source.ok_or_else(|| de::Error::missing_field(NODES_LEN_FIELD))?;
        let target = target.ok_or_else(|| de::Error::missing_field(ARROWS_FIELD))?;
        Ok(DirectedGraphDTO::new(source, target))
    }
}

impl<'de> Deserialize<'de> for DirectedGraphDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_struct(STRUCT_NAME, &[NODES_LEN_FIELD, ARROWS_FIELD], DirectedGraphDTOVisitor)
    }
}
