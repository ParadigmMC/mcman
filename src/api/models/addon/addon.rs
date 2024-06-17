use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::{app::App, models::Environment, step::Step, sources::resolve_steps_for_url};

use super::{AddonType, AddonTarget};

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
