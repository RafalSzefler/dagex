#![cfg(feature="serde")]

use serde::{de::Visitor, Deserialize, Serialize};

use super::TriBool;

const ERROR_MSG: &str = "Invalid TriBool value.";

impl Serialize for TriBool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        let value = self.as_u8();
        if value > 2 {
            return Err(serde::ser::Error::custom(ERROR_MSG));
        }
        serializer.serialize_u8(value)
    }
}

struct TriBoolVisitor;

impl<'de> Visitor<'de> for TriBoolVisitor {
    type Value = TriBool;
    
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("TriBool")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where E: serde::de::Error
    {
        match v {
            0 => Ok(TriBool::FALSE),
            1 => Ok(TriBool::UNKNOWN),
            2 => Ok(TriBool::TRUE),
            _ => Err(E::custom(ERROR_MSG))
        }       
    }
}

impl<'de> Deserialize<'de> for TriBool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        deserializer.deserialize_u8(TriBoolVisitor)
    }
}
