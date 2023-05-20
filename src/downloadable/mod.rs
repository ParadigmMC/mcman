use anyhow::Result;
use serde::{Deserialize, Serialize};

use self::{modrinth::fetch_modrinth, papermc::download_papermc_build, vanilla::fetch_vanilla};
mod modrinth;
mod papermc;
mod vanilla;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Downloadable {
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
        build: String,
    },
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
        }
    }

    pub fn get_server_filename(&self) -> String {
        match self {
            Self::Url { url } => url.split('/').last().unwrap().to_string(),
            Self::Vanilla { version } => format!("server-{}.jar", version),
            // Modrinth shouldnt even be available but whatever tabnine
            Self::Modrinth { id, version } => format!("{}-{}.jar", id, version),
            Self::PaperMC {
                project,
                version,
                build,
            } => format!("{}-{}-{}.jar", project, version, build),
        }
    }
}
