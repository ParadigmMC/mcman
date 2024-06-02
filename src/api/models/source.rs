use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::{app::App, models::ModpackSource};
use super::Addon;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Source {
    File {
        path: String,
    },

    Folder {
        path: String,
    },

    Modpack {
        #[serde(flatten)]
        modpack: ModpackSource,
    },
}

impl Source {
    pub async fn resolve(&self, app: &App) -> Result<Vec<Addon>> {
        match self {
            Source::File { path } => {
                Ok(vec![])
            }

            Source::Folder { path } => {
                Ok(vec![])
            }

            Source::Modpack { modpack } => {
                Ok(vec![])
            }
        }
    }
}
