use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

impl ToString for AddonType {
    fn to_string(&self) -> String {
        match self {
            AddonType::Url { url } => format!("Url [{url}]"),
            AddonType::Modrinth { id, version } => format!("Modrinth/{id} [{version}]"),
            AddonType::Curseforge { id, version } => format!("Curseforge/{id} [{version}]"),
            AddonType::Spigot { id, version } => format!("Spigot/{id} [{version}]"),
            AddonType::Hangar { id, version } => format!("Hangar/{id} [{version}]"),
            AddonType::GithubRelease {
                repo,
                version,
                filename,
            } => format!("Github/{repo} [{version}; {filename}]"),
            AddonType::Jenkins {
                url,
                job,
                build,
                artifact,
            } => format!("Jenkins/{job} [{build}; {artifact}]"),
            AddonType::MavenArtifact {
                url,
                group,
                artifact,
                version,
                filename,
            } => format!("Maven/{group}.{artifact} [{version}; {filename}]"),
        }
    }
}
