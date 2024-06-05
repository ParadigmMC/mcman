use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::{app::App, models::{Environment, Step}, sources::resolve_steps_for_url};

use super::AddonType;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
pub enum AddonTarget {
    Plugin,
    Mod,
    Custom(String),
}

impl Default for AddonTarget {
    fn default() -> Self {
        Self::Custom(String::new())
    }
}

impl AddonTarget {
    pub fn from_str(str: &str) -> Self {
        match str {
            "mods" => AddonTarget::Mod,
            "plugins" => AddonTarget::Plugin,
            other => AddonTarget::Custom(other.to_owned()),
        }
    }

    pub fn from_path(path: &str) -> Self {
        Self::from_str(&Path::new(path).parent().map(|p| p.to_string_lossy().into_owned()).unwrap_or(".".to_owned()))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
pub struct Addon {
    pub environment: Option<Environment>,
    #[serde(flatten)]
    pub addon_type: AddonType,
    pub target: AddonTarget,
}

impl Addon {
    pub async fn resolve_steps(&self, app: &App) -> Result<Vec<Step>> {
        match &self.addon_type {
            AddonType::Url { url } => resolve_steps_for_url(app, url, None).await,
            AddonType::Modrinth { id, version } => app.modrinth().resolve_steps(id, version).await,
            _ => Ok(vec![]),
        }
    }
}
