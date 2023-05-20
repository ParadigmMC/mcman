use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::error::Result;

use self::{vanilla::fetch_vanilla, modrinth::fetch_modrinth, papermc::download_papermc_build};
mod vanilla;
mod modrinth;
mod papermc;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Downloadable {
    URL { url: String },
    Vanilla { version: String },
    Modrinth { id: String, version: String },
    PaperMC { project: String, version: String, build: String },
}

impl Downloadable {
    pub async fn download(
        &self,
        client: &reqwest::Client,
    ) -> Result<Box<dyn futures_core::Stream<Item = reqwest::Result<Bytes>>>> {
        match self {
            Self::URL { url } => Ok(Box::new(client.get(url).send().await?.bytes_stream())),
            Self::Vanilla { version } => Ok(Box::new(fetch_vanilla(version, &client).await?)),
            Self::Modrinth { id, version } => Ok(Box::new(fetch_modrinth(id, version, &client).await?)),
            Self::PaperMC { project, version, build } => Ok(Box::new(download_papermc_build(project, version, build, &client).await?)),
        }
    }
}
