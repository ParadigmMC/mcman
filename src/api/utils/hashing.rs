use std::collections::HashMap;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum HashFormat {
    Sha256,
    Sha512,
    Sha1,
    Md5,
    #[serde(rename = "murmur2")]
    #[default]
    Curseforge,
}

impl TryFrom<String> for HashFormat {
    type Error = anyhow::Error;
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "sha256" => Ok(HashFormat::Sha256),
            "sha512" => Ok(HashFormat::Sha512),
            "sha1" => Ok(HashFormat::Sha1),
            "md5" => Ok(HashFormat::Md5),
            "murmur2" => Ok(HashFormat::Curseforge),
            fmt => Err(anyhow!("Unknown HashFormat {fmt}")),
        }
    }
}
