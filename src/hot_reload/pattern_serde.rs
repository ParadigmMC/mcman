use glob::Pattern;
use serde::{Serializer, Deserializer, Serialize, Deserialize};

pub fn serialize<S: Serializer>(pat: &Pattern, ser: S) -> Result<S::Ok, S::Error> {
    pat.as_str().serialize(ser)
}

pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Pattern, D::Error> {
    Pattern::new(&String::deserialize(de)?)
        .map_err(serde::de::Error::custom)
}
