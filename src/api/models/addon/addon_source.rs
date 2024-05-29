use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::models::ModpackSource;
use super::Addon;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AddonSource {
    File {
        path: String,
    },

    Folder {
        path: String,
    },

    Modpack {
        modpack: ModpackSource,
    },
}

impl AddonSource {
    pub async fn resolve(&self) -> Result<Vec<Addon>> {
        Ok(vec![])
    }
}
