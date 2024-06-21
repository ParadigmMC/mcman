use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::api::{app::App, sources::{jenkins::jenkins_job_url, maven::maven_artifact_url}, utils::url::get_filename_from_url};

use super::{Addon, AddonType};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum AddonMetadataSource {
    Modrinth,
    Curseforge,
    Hangar,
    Github,
    Spigot,
    Jenkins,
    Maven,
    #[default]
    Other,
}

impl AddonMetadataSource {
    pub fn into_str(&self) -> &'static str {
        match self {
            AddonMetadataSource::Modrinth => "modrinth",
            AddonMetadataSource::Hangar => "hangar",
            AddonMetadataSource::Spigot => "spigot",
            AddonMetadataSource::Other => "other",
            AddonMetadataSource::Github => "github",
            AddonMetadataSource::Jenkins => "jenkins",
            AddonMetadataSource::Maven => "maven",
            AddonMetadataSource::Curseforge => "curseforge",
        }
    }

    pub fn icon_url(&self) -> String {
        format!("https://raw.githubusercontent.com/ParadigmMC/mcman/main/res/icons/{}.png", self.into_str())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddonMetadata {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub link: Option<String>,
    pub source: AddonMetadataSource,
}

impl Addon {
    pub async fn resolve_metadata(&self, app: &App) -> Result<AddonMetadata> {
        match &self.addon_type {
            AddonType::Url { url } => Ok(AddonMetadata {
                name: get_filename_from_url(url),
                description: None,
                link: Some(url.to_owned()),
                source: AddonMetadataSource::Other,
                version: None,
            }),
            AddonType::Modrinth { id, version } => {
                let proj = app.modrinth().fetch_project(id).await?;

                Ok(AddonMetadata {
                    name: proj.title,
                    description: Some(proj.description),
                    version: Some(version.to_owned()),
                    link: Some(format!("https://modrinth.com/{}", proj.slug)),
                    source: AddonMetadataSource::Modrinth,
                })
            },
            AddonType::Curseforge { id, version } => todo!(),
            AddonType::Spigot { id, version } => todo!(),
            AddonType::Hangar { id, version } => {
                let proj = app.hangar().fetch_project(id).await?;

                Ok(AddonMetadata {
                    name: proj.name,
                    link: Some(format!("https://hangar.papermc.io/{}", proj.namespace.to_string())),
                    description: Some(proj.description),
                    source: AddonMetadataSource::Hangar,
                    version: Some(version.to_owned()),
                })
            },
            AddonType::GithubRelease {
                repo,
                version,
                ..
            } => {
                let desc = app.github().fetch_repo_description(repo).await?;

                Ok(AddonMetadata {
                    name: repo.to_owned(),
                    description: Some(desc),
                    link: Some(format!("https://github.com/{repo}")),
                    source: AddonMetadataSource::Github,
                    version: Some(version.to_owned()),
                })
            },
            AddonType::Jenkins {
                url,
                job,
                build,
                artifact,
            } => {
                let description = app.jenkins().fetch_description(url, job).await?;

                Ok(AddonMetadata {
                    name: artifact.to_owned(),
                    link: Some(jenkins_job_url(url, job)),
                    description,
                    source: AddonMetadataSource::Jenkins,
                    version: Some(build.to_owned()),
                })
            },
            AddonType::MavenArtifact {
                url,
                group,
                artifact,
                version,
                ..
            } => {
                Ok(AddonMetadata {
                    name: artifact.to_owned(),
                    link: Some(maven_artifact_url(url, group, artifact)),
                    description: None,
                    source: AddonMetadataSource::Maven,
                    version: Some(version.to_owned()),
                })
            },
        }
    }
}
