use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::sources::maven::{self, get_maven_url};
use crate::{model::Server, Source};

use crate::sources::{
    curserinth::{fetch_curserinth_filename, get_curserinth_url},
    github::{download_github_release, fetch_github_release_filename, get_github_release_url},
    jenkins::{get_jenkins_download_url, get_jenkins_filename},
    modrinth::{fetch_modrinth_filename, get_modrinth_url},
    spigot::{fetch_spigot_resource_latest_ver, get_spigot_url},
};
mod import_url;
mod markdown;
mod meta;
mod packwiz;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Downloadable {
    // sources
    Url {
        url: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "crate::util::is_default")]
        filename: Option<String>,
        #[serde(default)]
        #[serde(skip_serializing_if = "crate::util::is_default")]
        desc: Option<String>,
    },

    #[serde(alias = "mr")]
    Modrinth { id: String, version: String },

    #[serde(alias = "cr")]
    CurseRinth { id: String, version: String },

    Spigot {
        id: String, // weird ass api
    },

    #[serde(rename = "ghrel")]
    GithubRelease {
        repo: String,
        tag: String,
        asset: String,
    },

    // pain in the a-
    Jenkins {
        url: String,
        job: String,
        #[serde(default = "latest")]
        build: String,
        #[serde(default = "first")]
        artifact: String,
    },

    Maven {
        url: String,
        group: String,
        #[serde(default = "first")]
        artifact: String,
        #[serde(default = "latest")]
        version: String,
        #[serde(default = "artifact")]
        filename: String,
    },
}

pub fn latest() -> String {
    "latest".to_owned()
}

pub fn first() -> String {
    "first".to_owned()
}

pub fn artifact() -> String {
    "artifact".to_owned()
}

impl Downloadable {
    pub async fn get_url(
        &self,
        client: &reqwest::Client,
        server: &Server,
        filename_hint: Option<&str>,
    ) -> Result<String> {
        let mcver = &server.mc_version;

        match self {
            Self::Url { url, .. } => Ok(url.clone()),

            Self::Modrinth { id, version } => {
                Ok(get_modrinth_url(id, version, client, None).await?)
            }
            Self::CurseRinth { id, version } => {
                Ok(get_curserinth_url(id, version, client, None).await?)
            }
            Self::Spigot { id } => Ok(get_spigot_url(id)),
            Self::GithubRelease { repo, tag, asset } => {
                Ok(get_github_release_url(repo, tag, asset, mcver, client, filename_hint).await?)
            }

            Self::Jenkins {
                url,
                job,
                build,
                artifact,
            } => Ok(get_jenkins_download_url(client, url, job, build, artifact).await?),

            Self::Maven {
                url,
                group,
                artifact,
                version,
                filename,
            } => Ok(get_maven_url(client, url, group, artifact, version, filename, mcver).await?),
        }
    }
}

#[async_trait]
impl Source for Downloadable {
    async fn download(
        &self,
        server: &Server,
        client: &reqwest::Client,
        filename_hint: Option<&str>,
    ) -> Result<reqwest::Response> {
        match self {
            Self::GithubRelease { repo, tag, asset } => Ok(download_github_release(
                repo,
                tag,
                asset,
                &server.mc_version,
                client,
                filename_hint,
            )
            .await?),

            dl => Ok(client
                .get(dl.get_url(client, server, filename_hint).await?)
                .send()
                .await?
                .error_for_status()?),
        }
    }

    async fn get_filename(&self, server: &Server, client: &reqwest::Client) -> Result<String> {
        let mcver = &server.mc_version;

        match self {
            Self::Url { url, filename, .. } => {
                if let Some(filename) = filename {
                    return Ok(filename.clone());
                }

                let url_clean = url.split('?').next().unwrap_or(url);
                Ok(url_clean.split('/').last().unwrap().to_string())
            }

            Self::Modrinth { id, version } => {
                // nvm
                let filename = fetch_modrinth_filename(id, version, client, None).await?;
                Ok(filename)
            }
            Self::CurseRinth { id, version } => {
                let filename = fetch_curserinth_filename(id, version, client, None).await?;
                Ok(filename)
            }
            Self::Spigot { id } => {
                let ver = fetch_spigot_resource_latest_ver(id, client).await?;
                // amazing.. bruh...
                Ok(format!("{id}-{ver}.jar"))
            }

            // problematic stuff part 2345
            Self::GithubRelease { repo, tag, asset } => {
                Ok(fetch_github_release_filename(repo, tag, asset, mcver, client).await?)
            }

            Self::Jenkins {
                url,
                job,
                build,
                artifact,
            } => Ok(get_jenkins_filename(client, url, job, build, artifact)
                .await?
                .1),

            Self::Maven {
                url,
                group,
                artifact,
                version,
                filename,
            } => Ok(maven::get_maven_filename(
                client, url, group, artifact, version, filename, mcver,
            )
            .await?),
        }
    }
}

impl std::fmt::Display for Downloadable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Url { url, .. } => f.write_fmt(format_args!("Custom URL: {url}")),

            Self::Modrinth { id, version } => f
                .debug_struct("Modrinth")
                .field("id", id)
                .field("version", version)
                .finish(),

            Self::CurseRinth { id, version } => f
                .debug_struct("CurseRinth")
                .field("id", id)
                .field("version", version)
                .finish(),

            Self::Spigot { id } => f.write_fmt(format_args!("Spigot: {id}")),

            Self::GithubRelease { repo, tag, asset } => f
                .debug_struct("GithubRelease")
                .field("Repository", repo)
                .field("Tag/Release", tag)
                .field("Asset", asset)
                .finish(),

            Self::Jenkins {
                url,
                job,
                build,
                artifact,
            } => f
                .debug_struct("Jenkins")
                .field("Instance URL", url)
                .field("Job", job)
                .field("Build ID", build)
                .field("Artifact", artifact)
                .finish(),

            Self::Maven {
                url,
                group,
                artifact,
                version,
                filename,
            } => f
                .debug_struct("Maven")
                .field("Instance URL", url)
                .field("Group", group)
                .field("Artifact", artifact)
                .field("Version", version)
                .field("Filename", filename)
                .finish(),
        }
    }
}
