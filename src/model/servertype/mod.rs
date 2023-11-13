use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::{Resolvable, App, ResolvedFile};

use crate::model::Downloadable;
use crate::sources::quilt;

use super::StartupMethod;

pub mod interactive;
pub mod meta;
pub mod parse;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub enum SoftwareType {
    Normal,
    Modded,
    Proxy,
    #[default]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

    // TODO: move this to somewhere else, like BuildContext
    pub async fn get_install_method(
        &self,
        app: &App,
    ) -> Result<InstallMethod> {
        let mcver = &app.server.mc_version;
        Ok(match self.clone() {
            Self::Quilt { loader, .. } => {
                let mut args = vec!["install", "server", mcver];

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
                        "quilt-server-launch-{mcver}-{}.jar",
                        quilt::map_quilt_loader_version(&app.http_client, &loader)
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
                    "libraries/net/neoforged/forge/{mcver}-{0}/forge-{mcver}-{0}-server.jar",
                    app.neoforge().resolve_version(&loader).await?
                )
            },
            Self::Forge { loader } => InstallMethod::Installer {
                name: "Forge Installer".to_owned(),
                label: "fi".to_owned(),
                args: vec!["--installServer".to_owned(), ".".to_owned()],
                rename_from: None,
                jar_name: format!(
                    "libraries/net/minecraftforge/forge/{mcver}-{0}/forge-{mcver}-{0}-server.jar",
                    app.forge().resolve_version(&loader).await?
                )
            },
            Self::BuildTools { args, software } => {
                let mut buildtools_args = vec![
                    "--compile",
                    &software,
                    "--compile-if-changed",
                    "--rev",
                    mcver,
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
                        "{}-{mcver}.jar",
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

    // TODO: move this to somewhere else, like BuildContext
    pub async fn get_startup_method(
        &self,
        app: &App,
        serverjar_name: &str,
    ) -> Result<StartupMethod> {
        let mcver = &app.server.mc_version;
        Ok(match self {
            Self::NeoForge { loader } => {
                let l = app.neoforge().resolve_version(loader).await?;

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
                let l = app.forge().resolve_version(loader).await?;

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

    // TODO: move to ModrinthAPI
    pub fn get_modrinth_facets(&self, mcver: &str) -> Result<String> {
        let mut arr: Vec<Vec<String>> = vec![];

        if self.get_software_type() != SoftwareType::Proxy {
            arr.push(vec![format!("versions:{}", mcver.to_owned())]);
        }

        if let Some(n) = self.get_modrinth_name() {
            arr.push(vec![format!("categories:{n}")]);
        }

        Ok(serde_json::to_string(&arr)?)
    }

    // TODO: move to ModrinthAPI
    pub fn get_modrinth_name(&self) -> Option<String> {
        match self {
            Self::Fabric { .. } => Some("fabric"),
            Self::Quilt { .. } => Some("quilt"),
            Self::Forge { .. } => Some("forge"),
            Self::NeoForge { .. } => Some("neoforge"),
            Self::Paper {  } => Some("paper"),
            Self::BuildTools { .. } => Some("spigot"),
            Self::Purpur { .. } => Some("purpur"),
            Self::BungeeCord {  } => Some("bungeecord"),
            Self::Velocity {  } => Some("velocity"),
            Self::Waterfall {  } => Some("waterfall"),
            Self::PaperMC { project, .. } => Some(project.as_str()),
            _ => None,
        }.map(|o| o.to_owned())
    }

    // TODO: move to HangarAPI
    pub fn get_hangar_platform(&self) -> Option<mcapi::hangar::Platform> {
        match self {
            Self::Waterfall {} => Some(mcapi::hangar::Platform::Waterfall),
            Self::Velocity {} => Some(mcapi::hangar::Platform::Velocity),
            Self::PaperMC { project, .. } if project == "waterfall" => Some(mcapi::hangar::Platform::Waterfall),
            Self::PaperMC { project, .. } if project == "velocity" => Some(mcapi::hangar::Platform::Velocity),
            Self::PaperMC { project, .. } if project == "paper" => Some(mcapi::hangar::Platform::Paper),
            Self::Paper {  } | Self::Purpur { .. } => Some(mcapi::hangar::Platform::Paper),
            _ => None
        }
    }

    // TODO: move to HangarAPI
    pub fn get_hangar_versions_filter(&self, mcver: &str) -> mcapi::hangar::VersionsFilter {
        let platform = self.get_hangar_platform();
        mcapi::hangar::VersionsFilter {
            platform_version: if platform.is_some() {
                Some(mcver.to_owned())
            } else {
                None
            },
            platform,
            ..Default::default()
        }
    }

    pub fn is_modded(&self) -> bool {
        self.get_software_type() == SoftwareType::Modded
    }

    pub fn supports_eula_args(&self) -> bool {
        !matches!(self, Self::Vanilla {}) && !self.is_modded()
    }
}

impl ToString for ServerType {
    fn to_string(&self) -> String {
        match self {
            ServerType::Vanilla {  } => String::from("Vanilla"),
            ServerType::PaperMC { project, build } => format!("{project} build {build}"),
            ServerType::Purpur { build } => format!("Purpur build {build}"),
            ServerType::Fabric { loader, .. } => format!("Fabric v{loader}"),
            ServerType::Quilt { loader, .. } => format!("Quilt v{loader}"),
            ServerType::NeoForge { loader } => format!("NeoForge v{loader}"),
            ServerType::Forge { loader } => format!("Forge v{loader}"),
            ServerType::BuildTools { software, .. } => format!("(BuildTools) {software}"),
            ServerType::Paper {  } => format!("Paper"),
            ServerType::Velocity {  } => format!("Velocity"),
            ServerType::Waterfall {  } => format!("Waterfall"),
            ServerType::BungeeCord {  } => format!("BungeeCord"),
            ServerType::Downloadable { inner } => inner.to_string(),
        }
    }
}

#[async_trait]
impl Resolvable for ServerType {
    async fn resolve_source(&self, app: &App) -> Result<ResolvedFile> {
        let version = &app.mc_version();

        match self {
            ServerType::Vanilla {  } => app.vanilla().resolve_source(version).await,
            ServerType::PaperMC { project, build } => app.papermc().resolve_source(project, version, build).await,
            ServerType::Purpur { build } => app.purpur().resolve_source(version, build).await,
            ServerType::Fabric { loader, installer } => app.fabric().resolve_source(loader, installer).await,
            ServerType::Quilt { installer, .. } => app.quilt().resolve_installer(installer).await,
            ServerType::NeoForge { loader } => app.neoforge().resolve_source(loader).await,
            ServerType::Forge { loader } => app.forge().resolve_source(loader).await,
            ServerType::BuildTools { .. } => buildtools().resolve_source(app).await,
            ServerType::Paper {  } => app.papermc().resolve_source("paper", version, "latest").await,
            ServerType::Velocity {  } => app.papermc().resolve_source("velocity", version, "latest").await,
            ServerType::Waterfall {  } => app.papermc().resolve_source("waterfall", version, "latest").await,
            ServerType::BungeeCord {  } => bungeecord().resolve_source(app).await,
            ServerType::Downloadable { inner } => inner.resolve_source(app).await,
        }
    }
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
