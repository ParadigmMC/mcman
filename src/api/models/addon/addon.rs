use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::{app::App, models::Environment, sources::url::resolve_steps_for_url, step::Step};

use super::{AddonTarget, AddonType};

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
            AddonType::Curseforge { id, version } => todo!(),
            AddonType::Spigot { id, version } => todo!(),
            AddonType::Hangar { id, version } => todo!(),
            AddonType::GithubRelease { repo, version, filename } => app.github().resolve_steps(repo, version, filename).await,
            AddonType::Jenkins { url, job, build, artifact } => todo!(),
            AddonType::MavenArtifact { url, group, artifact, version, filename } => app.maven().resolve_steps(url, group, artifact, version, filename).await,
        }
    }
}
