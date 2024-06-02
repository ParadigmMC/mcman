use serde::{Deserialize, Serialize};
use crate::api::utils::serde::*;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BuildToolsFlavor {
    #[default]
    Spigot,
    CraftBukkit,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ServerType {
    Vanilla {
        mc_version: String,
    },

    PaperMC {
        project: String,
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

    Paper {},
    Velocity {},
    Waterfall {},
    BungeeCord {},

    Downloadable {
        //#[serde(flatten)]
        //inner: Downloadable,
    },
}
