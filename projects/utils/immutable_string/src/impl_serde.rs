use serde::{de::Visitor, Deserialize, Serialize};

use crate::ImmutableString;

impl Serialize for ImmutableString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        self.as_str().serialize(serializer)
    }
}

struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor {
    type Value = ImmutableString;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct ImmutableString")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
    {
        match ImmutableString::new(v) {
            Ok(imm) => Ok(imm),
            Err(err)
                => Err(E::custom(format!("Error on ImmutableString construction: {err:?}"))),
        }
    }
}

impl<'de> Deserialize<'de> for ImmutableString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        deserializer.deserialize_str(StrVisitor)
    }
}
