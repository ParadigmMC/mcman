use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::error::Result;

use self::vanilla::fetch_vanilla;
mod vanilla;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Downloadable {
    URL { url: String },
    Vanilla { version: String },
}

impl Downloadable {
    pub async fn download(
        &self,
        client: &reqwest::Client,
    ) -> Result<Box<dyn futures_core::Stream<Item = reqwest::Result<Bytes>>>> {
        match self {
            Self::URL { url } => Ok(Box::new(client.get(url).send().await?.bytes_stream())),
            Self::Vanilla { version } => Ok(Box::new(fetch_vanilla(version, &client).await?)),
        }
    }
}
