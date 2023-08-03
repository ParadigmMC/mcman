use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Source;

use crate::model::Downloadable;
use crate::sources::{
    fabric::{download_fabric, fetch_fabric_latest_installer, fetch_fabric_latest_loader},
    jenkins::{download_jenkins, get_jenkins_filename},
    papermc::{download_papermc_build, fetch_papermc_build},
    purpur::{download_purpurmc_build, fetch_purpurmc_builds},
    quilt::{download_quilt_installer, get_installer_filename, map_quilt_loader_version},
    vanilla::fetch_vanilla,
};

use super::{Server, StartupMethod};

pub mod interactive;
pub mod meta;

pub enum SoftwareType {
    Normal,
    Modded,
    Proxy,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Bridge {
    ServerType(ServerType),
    Downloadable(Downloadable),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ServerType {
    Vanilla {},

    PaperMC {
        project: String,
        #[serde(default = "latest")]
        #[serde(skip_serializing_if = "crate::util::is_default_str")]
        build: String,
    },

    Purpur {
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

    BuildTools {
        #[serde(default = "spigot")]
        software: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        args: Vec<String>,
    },

    Paper {},
    Velocity {},
    Waterfall {},
    BungeeCord {},

    Downloadable {
        #[serde(flatten)]
        inner: Downloadable,
    },
}

fn latest() -> String {
    "latest".to_owned()
}

fn spigot() -> String {
    "spigot".to_owned()
}

static BUNGEECORD_JENKINS: &str = "https://ci.md-5.net";
static BUNGEECORD_JOB: &str = "BungeeCord";
static BUNGEECORD_ARTIFACT: &str = "BungeeCord";
static BUILDTOOLS_JENKINS: &str = "https://hub.spigotmc.org/jenkins";

impl From<Bridge> for ServerType {
    fn from(value: Bridge) -> Self {
        match value {
            Bridge::ServerType(ty) => ty,
            Bridge::Downloadable(d) => Self::Downloadable { inner: d },
        }
    }
}

pub fn serialize<S>(st: &ServerType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    st.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<ServerType, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(ServerType::from(Bridge::deserialize(deserializer)?))
}

#[derive(Debug)]
pub enum InstallMethod {
    SingleJar,
    Installer {
        name: String,
        label: String,
        args: Vec<String>,
        rename_from: Option<String>,
        jar_name: String,
    },
}

impl ServerType {
    pub fn get_software_type(&self) -> SoftwareType {
        match self {
            Self::Velocity {} | Self::BungeeCord {} | Self::Waterfall {} => SoftwareType::Proxy,
            Self::PaperMC { project, .. } if project == "velocity" || project == "waterfall" => {
                SoftwareType::Proxy
            }
            Self::Quilt { .. } | Self::Fabric { .. } => SoftwareType::Modded,
            Self::Vanilla {}
            | Self::Paper {}
            | Self::PaperMC { .. }
            | Self::Purpur { .. }
            | Self::BuildTools { .. } => SoftwareType::Normal,
            Self::Downloadable { .. } => SoftwareType::Unknown,
        }
    }

    pub async fn get_install_method(&self, http_client: &reqwest::Client) -> Result<InstallMethod> {
        Ok(match self.clone() {
            Self::Quilt { loader, .. } => {
                let mut args = vec!["install", "server", "${mcver}"];

                if loader != "latest" {
                    args.push(&loader);
                }

                args.push("--install-dir=.");
                args.push("--download-server");

                InstallMethod::Installer {
                    name: "Quilt Server Installer".to_owned(),
                    label: "qsi".to_owned(),
                    args: args.into_iter().map(ToOwned::to_owned).collect(),
                    rename_from: Some("quilt-server-launch.jar".to_owned()),
                    jar_name: format!(
                        "quilt-server-launch-${{mcver}}-{}.jar",
                        map_quilt_loader_version(&http_client, &loader)
                            .await
                            .context("resolving quilt loader version id (latest/latest-beta)")?
                    ),
                }
            }
            Self::BuildTools { args, software } => {
                let mut buildtools_args = vec![
                    "--compile",
                    &software,
                    "--compile-if-changed",
                    "--rev",
                    "${mcver}",
                ];

                for arg in &args {
                    buildtools_args.push(arg);
                }

                InstallMethod::Installer {
                    name: "BuildTools".to_owned(),
                    label: "bt".to_owned(),
                    args: buildtools_args.into_iter().map(ToOwned::to_owned).collect(),
                    rename_from: Some("server.jar".to_owned()),
                    jar_name: format!(
                        "{}-${{mcver}}.jar",
                        if software == "craftbukkit" {
                            "craftbukkit"
                        } else {
                            "spigot"
                        }
                    ),
                }
            }
            _ => InstallMethod::SingleJar,
        })
    }

    pub fn get_startup_method(&self, serverjar_name: &str) -> StartupMethod {
        match self {
            //Self::Forge { .. } => StartupMethod::Custom(vec![""]),
            //Self::NeoForge { .. } => StartupMethod::Custom(vec!["@libraries/net/neoforged/forge/1.20.1-47.1.57/win_args.txt"]),
            _ => StartupMethod::Jar(serverjar_name.to_owned()),
        }
    }

    pub fn is_modded(&self) -> bool {
        matches!(self, Self::Fabric { .. } | Self::Quilt { .. })
    }

    pub fn supports_eula_args(&self) -> bool {
        !matches!(self, Self::Vanilla {}) && !self.is_modded()
    }
}

#[async_trait]
impl Source for ServerType {
    async fn download(
        &self,
        server: &Server,
        client: &reqwest::Client,
        filename_hint: Option<&str>,
    ) -> Result<reqwest::Response> {
        let mcver = server.mc_version.clone();
        match self {
            Self::Vanilla {} => Ok(fetch_vanilla(&mcver, client).await?),
            Self::PaperMC { project, build } => {
                Ok(download_papermc_build(project, &mcver, build, client).await?)
            }
            Self::Purpur { build } => Ok(download_purpurmc_build(&mcver, build, client).await?),

            Self::BungeeCord {} => Ok(download_jenkins(
                client,
                BUNGEECORD_JENKINS,
                BUNGEECORD_JOB,
                "latest",
                BUNGEECORD_ARTIFACT,
            )
            .await?),

            Self::BuildTools { .. } => Ok(download_jenkins(
                client,
                BUILDTOOLS_JENKINS,
                "BuildTools",
                "latest",
                "BuildTools",
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

            Self::Downloadable { inner } => inner.download(server, client, filename_hint).await,
        }
    }

    async fn get_filename(&self, server: &Server, client: &reqwest::Client) -> Result<String> {
        let mcver = server.mc_version.clone();
        match self {
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

            Self::BuildTools { .. } => {
                let build = get_jenkins_filename(
                    client,
                    BUILDTOOLS_JENKINS,
                    "BuildTools",
                    "latest",
                    "BuildTools",
                )
                .await?
                .3;
                Ok(format!("BuildTools-{build}.jar"))
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

            Self::Downloadable { inner } => inner.get_filename(server, client).await,
        }
    }
}

impl std::fmt::Display for ServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vanilla {} => f.write_str("Vanilla"),

            Self::Fabric { loader, installer } => f
                .debug_struct("Fabric")
                .field("loader", loader)
                .field("installer", installer)
                .finish(),

            Self::Quilt { loader, installer } => f
                .debug_struct("Quilt")
                .field("loader", loader)
                .field("installer", installer)
                .finish(),

            Self::BungeeCord {} => f.write_str("BungeeCord"),
            Self::BuildTools { .. } => f.write_str("BuildTools"),
            Self::Paper {} => f.write_str("Paper, latest"),
            Self::Velocity {} => f.write_str("Velocity, latest"),
            Self::Waterfall {} => f.write_str("Waterfall, latest"),
            Self::PaperMC { project, build } => {
                f.write_str(&format!("PaperMC/{project}, build {build}"))
            }
            Self::Purpur { build } => f.write_str(&format!("Purpur, build {build}")),
            Self::Downloadable { inner } => inner.fmt(f),
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
