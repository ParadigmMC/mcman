use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Source;

use crate::model::Downloadable;
use crate::sources::{fabric, forge, jenkins, neoforge, papermc, purpur, quilt, vanilla};

use super::{Server, StartupMethod};

pub mod interactive;
pub mod meta;

#[derive(Debug, PartialEq)]
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

    #[serde(alias = "neoforged")]
    NeoForge {
        #[serde(default = "latest")]
        loader: String,
    },

    Forge {
        #[serde(default = "latest")]
        loader: String,
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
            Self::Quilt { .. }
            | Self::Fabric { .. }
            | Self::NeoForge { .. }
            | Self::Forge { .. } => SoftwareType::Modded,
            Self::Vanilla {}
            | Self::Paper {}
            | Self::PaperMC { .. }
            | Self::Purpur { .. }
            | Self::BuildTools { .. } => SoftwareType::Normal,
            Self::Downloadable { .. } => SoftwareType::Unknown,
        }
    }

    pub async fn get_install_method(
        &self,
        http_client: &reqwest::Client,
        mcver: &str,
    ) -> Result<InstallMethod> {
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
                        quilt::map_quilt_loader_version(http_client, &loader)
                            .await
                            .context("resolving quilt loader version id (latest/latest-beta)")?
                    ),
                }
            }
            Self::NeoForge { loader } => InstallMethod::Installer {
                name: "NeoForged Installer".to_owned(),
                label: "nfi".to_owned(),
                args: vec!["--installServer".to_owned(), ".".to_owned()],
                rename_from: None,
                jar_name: format!(
                    "libraries/net/neoforged/forge/${{mcver}}-{0}/forge-${{mcver}}-{0}-server.jar",
                    neoforge::map_neoforge_version(&loader, mcver, http_client).await?
                )
            },
            Self::Forge { loader } => InstallMethod::Installer {
                name: "Forge Installer".to_owned(),
                label: "fi".to_owned(),
                args: vec!["--installServer".to_owned(), ".".to_owned()],
                rename_from: None,
                jar_name: format!(
                    "libraries/net/minecraftforge/forge/${{mcver}}-{0}/forge-${{mcver}}-{0}-server.jar",
                    forge::map_forge_version(&loader, mcver, http_client).await?
                )
            },
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

    pub async fn get_startup_method(
        &self,
        http_client: &reqwest::Client,
        serverjar_name: &str,
        mcver: &str,
    ) -> Result<StartupMethod> {
        Ok(match self {
            Self::NeoForge { loader } => {
                let l = neoforge::map_neoforge_version(loader, mcver, http_client).await?;

                StartupMethod::Custom {
                    windows: vec![format!(
                        "@libraries/net/neoforged/forge/{mcver}-{l}/win_args.txt"
                    )],
                    linux: vec![format!(
                        "@libraries/net/neoforged/forge/{mcver}-{l}/unix_args.txt"
                    )],
                }
            }
            Self::Forge { loader } => {
                let l = forge::map_forge_version(loader, mcver, http_client).await?;

                StartupMethod::Custom {
                    windows: vec![format!(
                        "@libraries/net/minecraftforge/forge/{mcver}-{l}/win_args.txt"
                    )],
                    linux: vec![format!(
                        "@libraries/net/minecraftforge/forge/{mcver}-{l}/unix_args.txt"
                    )],
                }
            }
            _ => StartupMethod::Jar(serverjar_name.to_owned()),
        })
    }

    pub fn is_modded(&self) -> bool {
        self.get_software_type() == SoftwareType::Modded
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
            Self::Vanilla {} => Ok(vanilla::fetch_vanilla(&mcver, client).await?),
            Self::PaperMC { project, build } => {
                Ok(papermc::download_papermc_build(project, &mcver, build, client).await?)
            }
            Self::Purpur { build } => {
                Ok(purpur::download_purpurmc_build(&mcver, build, client).await?)
            }

            Self::Paper {} => {
                Ok(papermc::download_papermc_build("paper", &mcver, "latest", client).await?)
            }
            Self::Velocity {} => {
                Ok(papermc::download_papermc_build("velocity", &mcver, "latest", client).await?)
            }
            Self::Waterfall {} => {
                Ok(papermc::download_papermc_build("waterfall", &mcver, "latest", client).await?)
            }

            Self::Fabric { loader, installer } => {
                Ok(fabric::download_fabric(client, &mcver, loader, installer).await?)
            }

            Self::Quilt { installer, .. } => {
                Ok(quilt::download_quilt_installer(client, installer).await?)
            }

            Self::BungeeCord {} => Ok(bungeecord().download(server, client, filename_hint).await?),
            Self::BuildTools { .. } => {
                Ok(buildtools().download(server, client, filename_hint).await?)
            }
            Self::NeoForge { loader } => Ok(client
                .get(neoforge::get_neoforge_installer_url(loader, &mcver, client).await?)
                .send()
                .await?
                .error_for_status()?),

            Self::Forge { loader } => Ok(client
                .get(forge::get_forge_installer_url(loader, &mcver, client).await?)
                .send()
                .await?
                .error_for_status()?),

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
                    let last_build = purpur::fetch_purpurmc_builds(&mcver, client)
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
                let build = jenkins::get_jenkins_filename(
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
                let build = jenkins::get_jenkins_filename(
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
                    "latest" => fabric::fetch_fabric_latest_loader(client).await?,
                    id => id.to_owned(),
                };

                let i = match installer.as_str() {
                    "latest" => fabric::fetch_fabric_latest_installer(client).await?,
                    id => id.to_owned(),
                };

                Ok(format!("fabric-server-{mcver}-{l}-{i}.jar"))
            }

            Self::Quilt { installer, .. } => {
                Ok(quilt::get_installer_filename(client, installer).await?)
            }

            Self::NeoForge { loader } => {
                Ok(neoforge::get_neoforge_installer_filename(loader, &mcver, client).await?)
            }

            Self::Forge { loader } => {
                Ok(forge::get_forge_installer_filename(loader, &mcver, client).await?)
            }

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

            Self::NeoForge { loader } => {
                f.debug_struct("NeoForged").field("loader", loader).finish()
            }

            Self::Forge { loader } => f.debug_struct("Forge").field("loader", loader).finish(),

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
        let build_id = papermc::fetch_papermc_build(project, mcver, build, client)
            .await?
            .build
            .to_string();
        Ok(format!("{project}-{mcver}-{build_id}.jar"))
    } else {
        Ok(format!("{project}-{mcver}-{build}.jar"))
    }
}

pub fn bungeecord() -> Downloadable {
    Downloadable::Jenkins {
        url: BUNGEECORD_JENKINS.to_owned(),
        job: BUNGEECORD_JOB.to_owned(),
        build: "latest".to_owned(),
        artifact: BUNGEECORD_ARTIFACT.to_owned(),
    }
}

pub fn buildtools() -> Downloadable {
    Downloadable::Jenkins {
        url: BUILDTOOLS_JENKINS.to_owned(),
        job: "BuildTools".to_owned(),
        build: "latest".to_owned(),
        artifact: "BuildTools".to_owned(),
    }
}
