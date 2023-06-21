use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::model::Server;

use self::sources::{
    github::{download_github_release, fetch_github_release_filename},
    modrinth::{download_modrinth, fetch_modrinth_filename},
    papermc::{download_papermc_build, fetch_papermc_build},
    purpur::{download_purpurmc_build, fetch_purpurmc_builds},
    spigot::{download_spigot_resource, fetch_spigot_resource_latest_ver},
    vanilla::fetch_vanilla, jenkins::{download_jenkins, get_jenkins_filename},
};
mod sources;
mod import_url;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Downloadable {
    // sources
    Url {
        url: String,
        #[serde(default)]
        #[serde(skip_serializing_if = "crate::util::is_default")]
        filename: Option<String>,
    },

    Vanilla {
        //version: String,
    },

    #[serde(alias = "mr")]
    Modrinth {
        id: String,
        version: String,
    },

    PaperMC {
        project: String,
        //version: String,
        #[serde(default = "latest")]
        #[serde(skip_serializing_if = "crate::util::is_default_str")]
        build: String,
    },

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

    // known projects
    Purpur {
        //version: String,
        #[serde(default = "latest")]
        build: String,
    },

    // papermc
    Paper {},
    Folia {},
    Velocity {},
    Waterfall {},
}

pub fn latest() -> String {
    "latest".to_owned()
}

pub fn first() -> String {
    "first".to_owned()
}

impl Downloadable {
    pub async fn download(
        &self,
        server: &Server,
        client: &reqwest::Client,
    ) -> Result<reqwest::Response> {
        let mcver = server.mc_version.clone();
        match self {
            Self::Url { url, filename: _ } => {
                Ok(client.get(url).send().await?.error_for_status()?)
            }

            Self::Vanilla {} => Ok(fetch_vanilla(&mcver, client).await?),
            Self::PaperMC { project, build } => {
                Ok(download_papermc_build(project, &mcver, build, client).await?)
            }
            Self::Purpur { build } => Ok(download_purpurmc_build(&mcver, build, client).await?),

            Self::Modrinth { id, version } => Ok(download_modrinth(id, version, client, None).await?),
            Self::Spigot { id } => Ok(download_spigot_resource(id, client).await?),
            Self::GithubRelease { repo, tag, asset } => {
                Ok(download_github_release(repo, tag, asset, client).await?)
            }

            Self::Jenkins { url, job, build, artifact } => Ok(
                download_jenkins(client, url, job, build, artifact).await?
            ),

            Self::Paper {} => Ok(download_papermc_build("paper", &mcver, "latest", client).await?),
            Self::Folia {} => Ok(download_papermc_build("folia", &mcver, "latest", client).await?),
            Self::Velocity {} => {
                Ok(download_papermc_build("velocity", &mcver, "latest", client).await?)
            }
            Self::Waterfall {} => {
                Ok(download_papermc_build("waterfall", &mcver, "latest", client).await?)
            }
        }
    }

    pub async fn get_filename(&self, server: &Server, client: &reqwest::Client) -> Result<String> {
        let mcver = server.mc_version.clone();
        match self {
            Self::Url { url, filename } => {
                if let Some(filename) = filename {
                    return Ok(filename.clone());
                }

                let url_clean = url.split('?').next().unwrap_or(url);
                Ok(url_clean.split('/').last().unwrap().to_string())
            }

            Self::Vanilla {} => Ok(format!("server-{mcver}.jar")),
            Self::PaperMC { project, build } => {
                Ok(get_filename_papermc(project, &mcver, build, client).await?)
            }
            Self::Purpur { build } => {
                if build == "latest" {
                    let last_build = fetch_purpurmc_builds(&mcver, client)
                        .await?
                        .last()
                        .cloned()
                        .unwrap_or("latest".to_owned());
                    Ok(format!("purpur-{mcver}-{last_build}.jar"))
                } else {
                    Ok(format!("purpur-{mcver}-{build}.jar"))
                }
            }

            Self::Modrinth { id, version } => {
                // nvm
                let filename = fetch_modrinth_filename(id, version, client, None).await?;
                Ok(filename)
            }
            Self::Spigot { id } => {
                let ver = fetch_spigot_resource_latest_ver(id, client).await?;
                // amazing.. bruh...
                Ok(format!("{id}-{ver}.jar"))
            }

            // problematic stuff part 2345
            Self::GithubRelease { repo, tag, asset } => {
                Ok(fetch_github_release_filename(repo, tag, asset, client).await?)
            }

            Self::Jenkins { url, job, build, artifact } => {
                Ok(get_jenkins_filename(client, url, job, build, artifact).await?.1)
            }

            Self::Paper {} => Ok(get_filename_papermc("paper", &mcver, "latest", client).await?),
            Self::Folia {} => Ok(get_filename_papermc("folia", &mcver, "latest", client).await?),
            Self::Velocity {} => {
                Ok(get_filename_papermc("velocity", &mcver, "latest", client).await?)
            }
            Self::Waterfall {} => {
                Ok(get_filename_papermc("waterfall", &mcver, "latest", client).await?)
            }
        }
    }
}

async fn get_filename_papermc(
    project: &str,
    mcver: &str,
    build: &str,
    client: &reqwest::Client,
) -> Result<String> {
    if build == "latest" {
        let build_id = fetch_papermc_build(project, mcver, build, client)
            .await?
            .build
            .to_string();
        Ok(format!("{project}-{mcver}-{build_id}.jar"))
    } else {
        Ok(format!("{project}-{mcver}-{build}.jar"))
    }
}
