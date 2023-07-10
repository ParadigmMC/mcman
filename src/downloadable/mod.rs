use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    downloadable::sources::fabric::{fetch_fabric_latest_installer, fetch_fabric_latest_loader},
    model::Server,
};

use self::sources::{
    fabric::download_fabric,
    github::{download_github_release, fetch_github_release_filename},
    jenkins::{download_jenkins, get_jenkins_filename},
    modrinth::{download_modrinth, fetch_modrinth_filename},
    papermc::{download_papermc_build, fetch_papermc_build},
    purpur::{download_purpurmc_build, fetch_purpurmc_builds},
    quilt::{download_quilt_installer, get_installer_filename},
    spigot::{download_spigot_resource, fetch_spigot_resource_latest_ver},
    vanilla::fetch_vanilla,
};
mod import_url;
mod interactive;
mod markdown;
pub mod sources;

static BUNGEECORD_JENKINS: &str = "https://ci.md-5.net";
static BUNGEECORD_JOB: &str = "BungeeCord";
static BUNGEECORD_ARTIFACT: &str = "BungeeCord";

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

    Fabric {
        #[serde(default = "latest")]
        loader: String,

        #[serde(default = "latest")]
        installer: String,
    },

    Quilt {
        #[serde(default = "latest")]
        loader: String,

        #[serde(default = "latest")]
        installer: String,
    },

    // papermc
    Paper {},
    Velocity {},
    Waterfall {},
    BungeeCord {},
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
            Self::Url { url, .. } => Ok(client.get(url).send().await?.error_for_status()?),

            Self::Vanilla {} => Ok(fetch_vanilla(&mcver, client).await?),
            Self::PaperMC { project, build } => {
                Ok(download_papermc_build(project, &mcver, build, client).await?)
            }
            Self::Purpur { build } => Ok(download_purpurmc_build(&mcver, build, client).await?),

            Self::Modrinth { id, version } => {
                Ok(download_modrinth(id, version, client, None).await?)
            }
            Self::Spigot { id } => Ok(download_spigot_resource(id, client).await?),
            Self::GithubRelease { repo, tag, asset } => {
                Ok(download_github_release(repo, tag, asset, client).await?)
            }

            Self::Jenkins {
                url,
                job,
                build,
                artifact,
            } => Ok(download_jenkins(client, url, job, build, artifact).await?),

            Self::BungeeCord {} => Ok(download_jenkins(
                client,
                BUNGEECORD_JENKINS,
                BUNGEECORD_JOB,
                "latest",
                BUNGEECORD_ARTIFACT,
            )
            .await?),

            Self::Paper {} => Ok(download_papermc_build("paper", &mcver, "latest", client).await?),
            Self::Velocity {} => {
                Ok(download_papermc_build("velocity", &mcver, "latest", client).await?)
            }
            Self::Waterfall {} => {
                Ok(download_papermc_build("waterfall", &mcver, "latest", client).await?)
            }

            Self::Fabric { loader, installer } => {
                Ok(download_fabric(client, &mcver, loader, installer).await?)
            }

            Self::Quilt { installer, .. } => Ok(download_quilt_installer(client, installer).await?),
        }
    }

    pub async fn get_filename(&self, server: &Server, client: &reqwest::Client) -> Result<String> {
        let mcver = server.mc_version.clone();
        match self {
            Self::Url { url, filename, .. } => {
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

            Self::Jenkins {
                url,
                job,
                build,
                artifact,
            } => Ok(get_jenkins_filename(client, url, job, build, artifact)
                .await?
                .1),

            Self::BungeeCord {} => {
                let build = get_jenkins_filename(
                    client,
                    BUNGEECORD_JENKINS,
                    BUNGEECORD_JOB,
                    "latest",
                    BUNGEECORD_ARTIFACT,
                )
                .await?
                .3;
                Ok(format!("BungeeCord-{build}.jar"))
            }

            Self::Paper {} => Ok(get_filename_papermc("paper", &mcver, "latest", client).await?),
            Self::Velocity {} => {
                Ok(get_filename_papermc("velocity", &mcver, "latest", client).await?)
            }
            Self::Waterfall {} => {
                Ok(get_filename_papermc("waterfall", &mcver, "latest", client).await?)
            }

            Self::Fabric { loader, installer } => {
                let l = match loader.as_str() {
                    "latest" => fetch_fabric_latest_loader(client).await?,
                    id => id.to_owned(),
                };

                let i = match installer.as_str() {
                    "latest" => fetch_fabric_latest_installer(client).await?,
                    id => id.to_owned(),
                };

                Ok(format!(
                    "fabric-server-mc.{mcver}-loader.{l}-launcher.{i}.jar"
                ))
            }

            Self::Quilt { installer, .. } => Ok(get_installer_filename(client, installer).await?),
        }
    }
}

impl std::fmt::Display for Downloadable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            Self::Url { url, .. } => {
                format!("Custom URL: {url}")
            }

            Self::Vanilla {} => "Vanilla".to_owned(),

            Self::Modrinth { id, version } => {
                format!(
                    "Modrinth Project {{
                    id: {id}
                    version: {version}
                }}"
                )
            }

            Self::Spigot { id } => format!("Spigot: {id}"),

            Self::GithubRelease { repo, tag, asset } => {
                format!(
                    "Github Release {{
                    Repository: {repo}
                    Release: {tag}
                    Asset: {asset}
                }}"
                )
            }

            Self::Jenkins {
                url,
                job,
                build,
                artifact,
            } => {
                format!(
                    "Jenkins {{
                    Jenkins URL: {url}
                    Job: {job}
                    Build: {build}
                    Artifact: {artifact}
                }}"
                )
            }

            Self::Fabric { loader, installer } => {
                format!(
                    "Fabric {{
                    Loader version: {loader}
                    Installer version: {installer}
                }}"
                )
            }

            Self::Quilt { loader, installer } => {
                format!(
                    "Quilt {{
                    Loader version: {loader}
                    Installer version: {installer}
                }}"
                )
            }

            Self::BungeeCord {} => "BungeeCord".to_owned(),
            Self::Paper {} => "Paper, latest".to_owned(),
            Self::Velocity {} => "Velocity, latest".to_owned(),
            Self::Waterfall {} => "Waterfall, latest".to_owned(),
            Self::PaperMC { project, build } => format!("PaperMC/{project}, build {build}"),
            Self::Purpur { build } => format!("Purpur, build {build}"),
        };
        f.write_str(&t)
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
