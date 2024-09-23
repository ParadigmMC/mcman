use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use curseforge::CurseforgeHasher;
use digest::DynDigest;
use md5::Digest;
use serde::{Deserialize, Serialize};

pub mod curseforge;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum HashFormat {
    #[default]
    Sha256,
    Sha512,
    Sha1,
    Md5,
    #[serde(rename = "murmur2")]
    Curseforge,
}

impl HashFormat {
    pub fn get_digest(self) -> Box<dyn DynDigest + Send> {
        match self {
            Self::Sha256 => Box::new(sha2::Sha256::new()),
            Self::Sha512 => Box::new(sha2::Sha512::new()),
            Self::Sha1 => Box::new(sha1::Sha1::new()),
            Self::Md5 => Box::new(md5::Md5::new()),
            Self::Curseforge => Box::new(CurseforgeHasher::new()),
        }
    }
}

impl FromStr for HashFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sha256" => Ok(Self::Sha256),
            "sha512" => Ok(Self::Sha512),
            "sha1" => Ok(Self::Sha1),
            "md5" => Ok(Self::Md5),
            "murmur2" => Ok(Self::Curseforge),
            fmt => Err(anyhow!("Unknown HashFormat {fmt}")),
        }
    }
}

pub fn get_best_hash(hashes: &HashMap<HashFormat, String>) -> Option<(HashFormat, String)> {
    hashes
        .get_key_value(&HashFormat::Sha512)
        .or(hashes.get_key_value(&HashFormat::Sha256))
        .or(hashes.get_key_value(&HashFormat::Md5))
        .or(hashes.get_key_value(&HashFormat::Sha1))
        .or(hashes.get_key_value(&HashFormat::Curseforge))
        .map(|(k, v)| (*k, v.clone()))
}
