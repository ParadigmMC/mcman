use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ModpackType {
    Packwiz,
    MRPack,
    Unsup,
}

impl fmt::Display for ModpackType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Packwiz => write!(f, "Packwiz"),
            Self::MRPack => write!(f, "MRPack"),
            Self::Unsup => write!(f, "Unsup"),
        }
    }
}
