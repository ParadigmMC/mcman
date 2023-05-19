use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::error::{Result, Error};

#[derive(Debug, Deserialize, Serialize)]
struct VersionManifestVersion {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct VersionManifestLatest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct VersionManifest {
    pub latest: VersionManifestLatest,
    pub versions: Vec<VersionManifestVersion>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PackageManifest {
    pub downloads: PackageManifestDownload,
}

// ? help

#[derive(Debug, Deserialize, Serialize)]
struct PackageManifestDownload {
    pub server: PackageManifestDownloadServer,
}

#[derive(Debug, Deserialize, Serialize)]
struct PackageManifestDownloadServer {
    pub url: String,
}

pub async fn fetch_vanilla(
    version: &str,
    client: &reqwest::Client,
) -> Result<impl futures_core::Stream<Item = reqwest::Result<Bytes>>> {
    let version_manifest: VersionManifest = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest.json")
        .send()
        .await?
        .json()
        .await?;

    let mut target_version = version;
    
    if target_version == "latest" {
        target_version = &version_manifest.latest.release;
    }

    if target_version == "latest-snapshot" {
        target_version = &version_manifest.latest.snapshot;
    }

    let verdata = version_manifest.versions
        .iter()
        .find(|&v| v.id == target_version);

    if verdata.is_none() {
        return Err(Error::VanillaVersionNotFound(target_version.to_string()));
    }

    let package_manifest: PackageManifest = client
        .get(&verdata.unwrap().url)
        .send()
        .await?
        .json()
        .await?;

    let req = client
        .get(package_manifest.downloads.server.url)
        .send()
        .await?
        .bytes_stream();

    Ok(req)
}
