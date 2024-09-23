use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::{
    app::App,
    models::Environment,
    sources::url::{resolve_remove_steps_for_url, resolve_steps_for_url},
    step::Step,
};

use super::{AddonTarget, AddonType};

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
pub struct Addon {
    #[serde(alias = "side")]
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
            AddonType::Curseforge { id, version } => {
                app.curseforge().resolve_steps(id, version).await
            },
            AddonType::Spigot { id, version } => app.spigot().resolve_steps(id, version).await,
            AddonType::Hangar { id, version } => app.hangar().resolve_steps(id, version).await,
            AddonType::GithubRelease {
                repo,
                version,
                filename,
            } => app.github().resolve_steps(repo, version, filename).await,
            AddonType::Jenkins {
                url,
                job,
                build,
                artifact,
            } => {
                app.jenkins()
                    .resolve_steps(url, job, build, artifact, None)
                    .await
            },
            AddonType::MavenArtifact {
                url,
                group,
                artifact,
                version,
                filename,
            } => {
                app.maven()
                    .resolve_steps(url, group, artifact, version, filename)
                    .await
            },
        }
    }

    pub async fn resolve_remove_steps(&self, app: &App) -> Result<Vec<Step>> {
        match &self.addon_type {
            AddonType::Url { url } => resolve_remove_steps_for_url(app, url, None).await,
            AddonType::Modrinth { id, version } => {
                app.modrinth().resolve_remove_steps(id, version).await
            },
            AddonType::Curseforge { id, version } => {
                app.curseforge().resolve_remove_steps(id, version).await
            },
            AddonType::Spigot { id, version } => {
                app.spigot().resolve_remove_steps(id, version).await
            },
            AddonType::Hangar { id, version } => {
                app.hangar().resolve_remove_steps(id, version).await
            },
            AddonType::GithubRelease {
                repo,
                version,
                filename,
            } => {
                app.github()
                    .resolve_remove_steps(repo, version, filename)
                    .await
            },
            AddonType::Jenkins {
                url,
                job,
                build,
                artifact,
            } => {
                app.jenkins()
                    .resolve_remove_steps(url, job, build, artifact, None)
                    .await
            },
            AddonType::MavenArtifact {
                url,
                group,
                artifact,
                version,
                filename,
            } => {
                app.maven()
                    .resolve_remove_steps(url, group, artifact, version, filename)
                    .await
            },
        }
    }
}
