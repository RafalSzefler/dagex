use std::collections::HashMap;

use immutable_string::ImmutableString;
use serde::{de::{self, Visitor}, ser::SerializeStruct, Deserialize, Serialize};

use crate::{core::DirectedGraphDTO, phylo::PhylogeneticNetworkDTO};

const STRUCT_NAME: &str = "PhylogeneticNetworkDTO";
const NODES_LEN_FIELD: &str = "number_of_nodes";
const ARROWS_FIELD: &str = "arrows";
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
        let mut state = serializer.serialize_struct(STRUCT_NAME, 3)?;
        let graph = self.graph();
        state.serialize_field(NODES_LEN_FIELD, &graph.number_of_nodes())?;
        state.serialize_field(ARROWS_FIELD, &graph.arrows())?;
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
        let no = seq.next_element()?.unwrap();
        let arrows = seq.next_element()?.unwrap();
        let taxa = seq.next_element()?.unwrap();
        Ok(PhylogeneticNetworkDTO::new(DirectedGraphDTO::new(no, arrows), taxa))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
    {
        let mut no = None;
        let mut arrows = None;
        let mut raw_taxa: Option<Vec<(i32, ImmutableString)>> = None;
        while let Some(key) = map.next_key()? {
            match key {
                NODES_LEN_FIELD => {
                    if no.is_some() {
                        return Err(de::Error::duplicate_field(NODES_LEN_FIELD));
                    }
                    no = Some(map.next_value()?);
                },
                ARROWS_FIELD => {
                    if arrows.is_some() {
                        return Err(de::Error::duplicate_field(ARROWS_FIELD));
                    }
                    arrows = Some(map.next_value()?);
                },
                TAXA_FIELD => {
                    if raw_taxa.is_some() {
                        return Err(de::Error::duplicate_field(TAXA_FIELD));
                    }
                    raw_taxa = Some(map.next_value()?);
                },
                _ => { }
            }
        }

        let no = no.ok_or_else(|| de::Error::missing_field(NODES_LEN_FIELD))?;
        let arrows = arrows.ok_or_else(|| de::Error::missing_field(ARROWS_FIELD))?;
        let raw_taxa = raw_taxa.ok_or_else(|| de::Error::missing_field(TAXA_FIELD))?;
        let mut taxa = HashMap::with_capacity(raw_taxa.len());
        for (node, imm) in raw_taxa {
            if taxa.insert(node, imm).is_some() {
                return Err(de::Error::custom("Taxa contains duplicate keys."));
            }
        }
        Ok(PhylogeneticNetworkDTO::new(DirectedGraphDTO::new(no, arrows), taxa))
    }
}

impl<'de> Deserialize<'de> for PhylogeneticNetworkDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_struct(STRUCT_NAME, &[NODES_LEN_FIELD, ARROWS_FIELD, TAXA_FIELD], DirectedGraphDTOVisitor)
    }
}
