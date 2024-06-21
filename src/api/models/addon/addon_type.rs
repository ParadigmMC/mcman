use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
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
