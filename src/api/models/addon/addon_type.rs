use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::Addon;

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
#[non_exhaustive]
pub enum AddonType {
    Url {
        url: String,
    },

    #[serde(alias = "mr")]
    Modrinth {
        id: String,
        version: String,
    },

    #[serde(alias = "cf")]
    Curseforge {
        id: String,
        version: String,
    },

    Spigot {
        id: String,
        version: String,
    },

    Hangar {
        id: String,
        version: String,
    },

    #[serde(alias = "ghrel")]
    GithubRelease {
        #[serde(alias = "repository")]
        repo: String,
        #[serde(alias = "release")]
        #[serde(alias = "tag")]
        version: String,
        #[serde(alias = "asset")]
        filename: String,
    },

    Jenkins {
        url: String,
        job: String,
        build: String,
        artifact: String,
    },

    MavenArtifact {
        url: String,
        group: String,
        artifact: String,
        version: String,
        filename: String,
    },
}

impl fmt::Display for AddonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Url { url } => write!(f, "Url [{url}]"),
            Self::Modrinth { id, version } => write!(f, "Modrinth/{id} [{version}]"),
            Self::Curseforge { id, version } => write!(f, "Curseforge/{id} [{version}]"),
            Self::Spigot { id, version } => write!(f, "Spigot/{id} [{version}]"),
            Self::Hangar { id, version } => write!(f, "Hangar/{id} [{version}]"),
            Self::GithubRelease {
                repo,
                version,
                filename,
            } => write!(f, "Github/{repo} [{version}; {filename}]"),
            Self::Jenkins {
                url,
                job,
                build,
                artifact,
            } => write!(f, "Jenkins/{job} [{build}; {artifact}]"),
            Self::MavenArtifact {
                url,
                group,
                artifact,
                version,
                filename,
            } => write!(f, "Maven/{group}.{artifact} [{version}; {filename}]"),
        }
    }
}
