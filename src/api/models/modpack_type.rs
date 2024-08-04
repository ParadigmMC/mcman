use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ModpackType {
    Packwiz,
    MRPack,
    Unsup,
}

impl ToString for ModpackType {
    fn to_string(&self) -> String {
        match self {
            ModpackType::Packwiz => String::from("Packwiz"),
            ModpackType::MRPack => String::from("MRPack"),
            ModpackType::Unsup => String::from("Unsup"),
        }
    }
}
