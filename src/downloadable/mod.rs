use anyhow::Result;
use serde::{Deserialize, Serialize};

use self::{modrinth::fetch_modrinth, papermc::download_papermc_build, vanilla::fetch_vanilla, purpur::download_purpurmc_build};
mod modrinth;
mod papermc;
mod vanilla;
mod purpur;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Downloadable {
    // sources
    Url {
        url: String,
    },
    Vanilla {
        version: String,
    },
    Modrinth {
        id: String,
        version: String,
    },
    PaperMC {
        project: String,
        version: String,
        #[serde(default = "latest")]
        build: String,
    },
    
    // known projects
    Purpur {
        version: String,
        #[serde(default = "latest")]
        build: String,
    },

    // papermc
    Paper { version: String },
    Folia { version: String },
    Velocity { version: String },
    Waterfall { version: String },
}

pub fn latest() -> String {
    "latest".to_owned()
}

impl Downloadable {
    pub async fn download(&self, client: &reqwest::Client) -> Result<reqwest::Response> {
        match self {
            Self::Url { url } => Ok(client.get(url).send().await?.error_for_status()?),
            Self::Vanilla { version } => Ok(fetch_vanilla(version, client).await?),
            Self::Modrinth { id, version } => Ok(fetch_modrinth(id, version, client).await?),
            Self::PaperMC {
                project,
                version,
                build,
            } => Ok(download_papermc_build(project, version, build, client).await?),
            Self::Purpur { version, build } => Ok(download_purpurmc_build(version, build, client).await?),

            Self::Paper { version } => Ok(download_papermc_build("paper", version, "latest", client).await?),
            Self::Folia { version } => Ok(download_papermc_build("folia", version, "latest", client).await?),
            Self::Velocity { version } => Ok(download_papermc_build("velocity", version, "latest", client).await?),
            Self::Waterfall { version } => Ok(download_papermc_build("waterfall", version, "latest", client).await?),
        }
    }

    pub fn get_filename(&self) -> String {
        match self {
            Self::Url { url } => url.split('/').last().unwrap().to_string(),
            Self::Vanilla { version } => format!("server-{version}.jar"),
            Self::Modrinth { id, version } => format!("{id}-{version}.jar"),
            Self::PaperMC {
                project,
                version,
                build,
            } => format!("{project}-{version}-{build}.jar"),
            Self::Purpur { version, build } => format!("purpur-{version}-{build}.jar"),

            Self::Paper { version } => format!("paper-{version}.jar"),
            Self::Folia { version } => format!("folia-{version}.jar"),
            Self::Velocity { version } => format!("velocity-{version}.jar"),
            Self::Waterfall { version } => format!("waterfall-{version}.jar"),
        }
    }
}
