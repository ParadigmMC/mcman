use crate::api::{
    app::App,
    models::{Addon, AddonTarget, AddonType, Environment},
    step::Step,
    utils::serde::*,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BuildToolsFlavor {
    #[default]
    Spigot,
    CraftBukkit,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ServerJar {
    pub mc_version: String,
    #[serde(flatten)]
    pub server_type: ServerType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
        #[serde(default = "BuildToolsFlavor::default")]
        software: BuildToolsFlavor,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        #[serde(default = "Vec::new")]
        args: Vec<String>,
    },

    Custom {
        #[serde(flatten)]
        inner: AddonType,
    },
}

impl ServerJar {
    pub async fn resolve_steps(&self, app: &App, env: Environment) -> Result<Vec<Step>> {
        match &self.server_type {
            ServerType::Vanilla {} => app.vanilla().resolve_steps(&self.mc_version, env).await,
            ServerType::PaperMC { project, build } => app.papermc().resolve_steps(&project.to_string(), &self.mc_version, build).await,
            ServerType::Purpur { build } => todo!(),
            ServerType::Fabric { loader, installer } => app.fabric().resolve_steps(&self.mc_version, loader, installer, &env).await,
            ServerType::Quilt { loader, installer } => todo!(),
            ServerType::NeoForge { loader } => todo!(),
            ServerType::Forge { loader } => todo!(),
            ServerType::BuildTools { software, args } => todo!(),
            ServerType::Custom { inner } => {
                Addon {
                    environment: Some(env),
                    addon_type: inner.clone(),
                    target: AddonTarget::Custom(String::from(".")),
                }.resolve_steps(app).await
            },
        }
    }
}
