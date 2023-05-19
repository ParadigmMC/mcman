use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::error::Result;
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
    ) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>> {
        match self {
            Self::URL { url } => Ok(client.get(url).send().await?.bytes_stream()),
            Self::Vanilla { version } => Ok(())
        }
    }
}
