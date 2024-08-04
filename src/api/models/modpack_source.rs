use std::{path::Path, str::FromStr};

use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::api::utils::accessor::Accessor;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum ModpackSource {
    Local {
        path: String,
    },

    Remote {
        url: String,
    },
}

impl FromStr for ModpackSource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.starts_with("http") {
            Ok(ModpackSource::Remote { url: s.into() })
        } else {
            Ok(ModpackSource::Local { path: s.into() })
        }
    }
}

impl ModpackSource {
    pub fn accessor(&self, base: &Path) -> Result<Accessor> {
        let str = match self {
            Self::Local { path } => &base.join(path).to_string_lossy().into_owned(),
            Self::Remote { url } => url,
        };

        Ok(Accessor::from(str)?)
    }
}

