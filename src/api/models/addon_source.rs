use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{ModpackSource, Addon};


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
    async fn resolve(&self) -> Result<Vec<Addon>> {
        Ok(vec![])
    }
}
