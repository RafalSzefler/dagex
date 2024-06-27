use std::collections::HashMap;

use immutable_string::ImmutableString;
use serde::{de::{self, Visitor}, ser::SerializeStruct, Deserialize, Serialize};

use crate::phylo::PhylogeneticNetworkDTO;

const STRUCT_NAME: &str = "PhylogeneticNetworkDTO";
const GRAPH_FIELD: &str = "graph";
const TAXA_FIELD: &str = "taxa";

impl Serialize for PhylogeneticNetworkDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let mut taxa_content: Vec<(i32, &ImmutableString)> = self.taxa()
            .iter()
            .map(|p| (*p.0, p.1))
            .collect();
        taxa_content.sort_by(|l, r| l.0.cmp(&r.0));
        let mut state = serializer.serialize_struct(STRUCT_NAME, 2)?;
        state.serialize_field(GRAPH_FIELD, &self.graph())?;
        state.serialize_field(TAXA_FIELD, &taxa_content)?;
        state.end()
    }
}

struct DirectedGraphDTOVisitor;

impl<'de> Visitor<'de> for DirectedGraphDTOVisitor {
    type Value = PhylogeneticNetworkDTO;

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
        Ok(PhylogeneticNetworkDTO::new(source, target))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
    {
        let mut source = None;
        let mut target: Option<Vec<(i32, ImmutableString)>> = None;
        while let Some(key) = map.next_key()? {
            match key {
                GRAPH_FIELD => {
                    if source.is_some() {
                        return Err(de::Error::duplicate_field(GRAPH_FIELD));
                    }
                    source = Some(map.next_value()?);
                },
                TAXA_FIELD => {
                    if target.is_some() {
                        return Err(de::Error::duplicate_field(TAXA_FIELD));
                    }
                    target = Some(map.next_value()?);
                },
                _ => { }
            }
        }

        let source = source.ok_or_else(|| de::Error::missing_field(GRAPH_FIELD))?;
        let target = target.ok_or_else(|| de::Error::missing_field(TAXA_FIELD))?;
        let mut taxa = HashMap::with_capacity(target.len());
        for (node, imm) in target {
            if taxa.insert(node, imm).is_some() {
                return Err(de::Error::custom("Taxa contains duplicate keys."));
            }
        }
        Ok(PhylogeneticNetworkDTO::new(source, taxa))
    }
}

impl<'de> Deserialize<'de> for PhylogeneticNetworkDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_struct(STRUCT_NAME, &[GRAPH_FIELD, TAXA_FIELD], DirectedGraphDTOVisitor)
    }
}
