use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Deserialize, Serialize)]
struct VersionManifestVersion {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct VersionManifest {
    pub versions: Vec<VersionManifestVersion>,
}

async fn fetch_vanilla(
    version: String,
    client: &reqwest::Client,
) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>> {
    let version_manifest: VersionManifest = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest.json")
        .send()
        .await?
        .json()
        .await?;
    

    Ok(())
}
