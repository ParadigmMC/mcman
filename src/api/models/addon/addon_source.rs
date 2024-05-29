use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::{app::App, models::ModpackSource};
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
    pub async fn resolve(&self, app: &App) -> Result<Vec<Addon>> {
        match self {
            AddonSource::File { path } => {
                Ok(vec![])
            }

            AddonSource::Folder { path } => {
                Ok(vec![])
            }

            AddonSource::Modpack { modpack } => {
                Ok(vec![])
            }
        }
    }
}
