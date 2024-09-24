use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::api::{
    app::App,
    models::{Addon, AddonType},
    sources::{jenkins::jenkins_job_url, maven::maven_artifact_url},
    utils::url::get_filename_from_url,
};

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
    pub fn all() -> Vec<Self> {
        vec![
            AddonMetadataSource::Modrinth,
            AddonMetadataSource::Hangar,
            AddonMetadataSource::Spigot,
            AddonMetadataSource::Other,
            AddonMetadataSource::Github,
            AddonMetadataSource::Jenkins,
            AddonMetadataSource::Maven,
            AddonMetadataSource::Curseforge,
        ]
    }

    pub fn get_markdown_header() -> String {
        Self::all()
            .into_iter()
            .map(|src| format!("[{}]: {}", src.into_str(), src.icon_url()))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn into_str(self) -> &'static str {
        match self {
            Self::Modrinth => "modrinth",
            Self::Hangar => "hangar",
            Self::Spigot => "spigot",
            Self::Other => "other",
            Self::Github => "github",
            Self::Jenkins => "jenkins",
            Self::Maven => "maven",
            Self::Curseforge => "curseforge",
        }
    }

    pub fn markdown_tag(self) -> String {
        format!("![{}]", self.into_str())
    }

    pub fn html(self) -> String {
        format!("<img src='{}'>", self.icon_url())
    }

    pub fn icon_url(self) -> String {
        // TODO !!!!!!!!!!!!!!!!!!!!!
        format!(
            "https://raw.githubusercontent.com/ParadigmMC/mcman/v2/res/icons/{}.png",
            self.into_str()
        )
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
        Ok(match &self.addon_type {
            AddonType::Url { url } => AddonMetadata {
                name: get_filename_from_url(url),
                description: None,
                link: Some(url.to_owned()),
                source: AddonMetadataSource::Other,
                version: None,
            },
            AddonType::Modrinth { id, version } => {
                let proj = app.modrinth().fetch_project(id).await?;

                AddonMetadata {
                    name: proj.title,
                    description: Some(proj.description),
                    version: Some(version.to_owned()),
                    link: Some(format!("https://modrinth.com/{}", proj.slug)),
                    source: AddonMetadataSource::Modrinth,
                }
            },
            AddonType::Curseforge { id, version } => {
                let m = app.curseforge().fetch_mod(id).await?;

                AddonMetadata {
                    name: m.name,
                    description: Some(m.summary),
                    version: Some(version.to_owned()),
                    link: Some(m.links.website_url),
                    source: AddonMetadataSource::Curseforge,
                }
            },
            AddonType::Spigot { id, version } => {
                let r = app.spigot().fetch_resource(id).await?;

                AddonMetadata {
                    name: r.name,
                    description: Some(r.tag),
                    version: Some(version.to_owned()),
                    link: Some(format!("https://www.spigotmc.org/resources/{id}")),
                    source: AddonMetadataSource::Curseforge,
                }
            },
            AddonType::Hangar { id, version } => {
                let proj = app.hangar().fetch_project(id).await?;

                AddonMetadata {
                    name: proj.name,
                    link: Some(format!("https://hangar.papermc.io/{}", proj.namespace)),
                    description: Some(proj.description),
                    source: AddonMetadataSource::Hangar,
                    version: Some(version.to_owned()),
                }
            },
            AddonType::GithubRelease { repo, version, .. } => {
                let desc = app.github().fetch_repo_description(repo).await?;

                AddonMetadata {
                    name: repo.to_owned(),
                    description: Some(desc),
                    link: Some(format!("https://github.com/{repo}")),
                    source: AddonMetadataSource::Github,
                    version: Some(version.to_owned()),
                }
            },
            AddonType::Jenkins {
                url,
                job,
                build,
                artifact,
            } => {
                let description = app.jenkins().fetch_description(url, job).await?;

                AddonMetadata {
                    name: artifact.to_owned(),
                    link: Some(jenkins_job_url(url, job)),
                    description,
                    source: AddonMetadataSource::Jenkins,
                    version: Some(build.to_owned()),
                }
            },
            AddonType::MavenArtifact {
                url,
                group,
                artifact,
                version,
                ..
            } => AddonMetadata {
                name: artifact.to_owned(),
                link: Some(maven_artifact_url(url, group, artifact)),
                description: None,
                source: AddonMetadataSource::Maven,
                version: Some(version.to_owned()),
            },
        })
    }
}
