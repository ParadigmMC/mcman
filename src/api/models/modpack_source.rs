use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::api::utils::accessor::Accessor;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum ModpackSource {
    Local {
        modpack_type: ModpackType,
        path: String,
    },

    Remote {
        modpack_type: ModpackType,
        url: String,
    },
}

impl ModpackSource {
    pub fn accessor(&self) -> Result<Accessor> {
        let str = match self {
            Self::Local { path, .. } => path,
            Self::Remote { url, .. } => url,
        };

        Ok(Accessor::from(str)?)
    }

    pub fn modpack_type(&self) -> ModpackType {
        match self {
            Self::Local { modpack_type, .. } => *modpack_type,
            Self::Remote { modpack_type, .. } => *modpack_type,
        }
    }
}

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
