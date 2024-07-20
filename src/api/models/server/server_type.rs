use crate::api::{
    app::App, models::{Addon, AddonTarget, AddonType, Environment}, sources::buildtools, step::Step, utils::serde::*
};
use anyhow::Result;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::ServerFlavor;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum PaperMCProject {
    #[default]
    #[serde(alias = "PAPER")]
    Paper,
    #[serde(alias = "VELOCITY")]
    Velocity,
    #[serde(alias = "WATERFALL")]
    Waterfall,
}

impl ToString for PaperMCProject {
    fn to_string(&self) -> String {
        match self {
            Self::Paper => "paper".to_owned(),
            Self::Waterfall => "waterfall".to_owned(),
            Self::Velocity => "velocity".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ServerJar {
    pub mc_version: String,
    #[serde(flatten)]
    pub server_type: ServerType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ServerType {
    Vanilla {},

    PaperMC {
        project: PaperMCProject,
        #[serde(default = "str_latest")]
        build: String,
    },

    Purpur {
        #[serde(default = "str_latest")]
        build: String,
    },

    Fabric {
        #[serde(default = "str_latest")]
        loader: String,

        #[serde(default = "str_latest")]
        installer: String,
    },

    Quilt {
        #[serde(default = "str_latest")]
        loader: String,

        #[serde(default = "str_latest")]
        installer: String,
    },

    NeoForge {
        #[serde(default = "str_latest")]
        loader: String,
    },

    Forge {
        #[serde(default = "str_latest")]
        loader: String,
    },

    BuildTools {
        #[serde(default)]
        craftbukkit: bool,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(default = "Vec::<String>::new")]
        args: Vec<String>,
    },

    Custom {
        #[serde(flatten)]
        inner: AddonType,

        #[serde(default)]
        flavor: ServerFlavor,
        
        #[serde(default)]
        exec: Option<String>,
    },
}

impl ServerJar {
    pub fn flavor(&self) -> ServerFlavor {
        match &self.server_type {
            ServerType::BuildTools { .. }
            | ServerType::PaperMC { .. }
            | ServerType::Purpur { .. } => ServerFlavor::Patched,
            ServerType::Custom { flavor, .. } => flavor.clone(),
            ServerType::Fabric { .. }
            | ServerType::Forge { .. }
            | ServerType::Quilt { .. }
            | ServerType::NeoForge { .. } => ServerFlavor::Modded,
            ServerType::Vanilla {  } => ServerFlavor::Vanilla,
        }
    }

    pub async fn resolve_steps(&self, app: &App, env: Environment) -> Result<Vec<Step>> {
        match &self.server_type {
            ServerType::Vanilla {} => app.vanilla().resolve_steps(&self.mc_version, env).await,
            ServerType::PaperMC { project, build } => app.papermc().resolve_steps(&project.to_string(), &self.mc_version, build).await,
            ServerType::Purpur { build } => todo!(),
            ServerType::Fabric { loader, installer } => app.fabric().resolve_steps(&self.mc_version, loader, installer, &env).await,
            ServerType::Quilt { loader, installer } => app.quilt().resolve_steps(&self.mc_version, installer, loader, &env).await,
            ServerType::NeoForge { loader } => todo!(),
            ServerType::Forge { loader } => todo!(),
            ServerType::BuildTools { craftbukkit, args } => {
                buildtools::resolve_steps(app, *craftbukkit, args, &self.mc_version).await
            },
            ServerType::Custom { inner, .. } => {
                Addon {
                    environment: Some(env),
                    addon_type: inner.clone(),
                    target: AddonTarget::Custom(String::from(".")),
                }.resolve_steps(app).await
            },
        }
    }

    pub fn get_execution_arguments(&self) -> Vec<String> {
        match &self.server_type {
            ServerType::Forge { .. } => todo!(),
            ServerType::Custom { exec, .. } => exec.clone()
                .unwrap_or(String::from("-jar server.jar"))
                .split_whitespace()
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>(),
            _ => vec![String::from("-jar"), String::from("server.jar")],
        }
    }

    pub async fn update(&mut self, app: &App) -> Result<bool> {
        match self.server_type.clone() {
            ServerType::Vanilla {  } => Ok(false),
            ServerType::PaperMC { project, build } => {
                let new_build = app.papermc().fetch_builds(&project.to_string(), &self.mc_version).await?
                    .builds
                    .into_iter()
                    .map(|b| b.build)
                    .max()
                    .unwrap()
                    .to_string();

                self.server_type = ServerType::PaperMC { project: project.clone(), build: new_build.clone() };

                Ok(new_build != build)
            },
            ServerType::Purpur { build } => todo!(),
            ServerType::Fabric { loader, installer } => {
                let latest_loader = app.fabric().fetch_loaders().await?.first().unwrap().version.clone();
                let latest_installer = app.fabric().fetch_installers().await?.first().unwrap().version.clone();

                self.server_type = ServerType::Fabric { loader: latest_loader.clone(), installer: latest_installer.clone() };

                Ok(loader != latest_loader || installer != latest_installer)
            },
            ServerType::Quilt { loader, installer } => todo!(),
            ServerType::NeoForge { loader } => todo!(),
            ServerType::Forge { loader } => todo!(),
            ServerType::BuildTools { .. } => Ok(false),
            ServerType::Custom { .. } => todo!(),
        }
    }
}
