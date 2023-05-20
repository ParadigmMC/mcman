use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
struct PackageManifestDownload {
    pub server: PackageManifestDownloadServer,
}

#[derive(Debug, Deserialize, Serialize)]
struct PackageManifestDownloadServer {
    pub url: String,
}

pub async fn fetch_vanilla(version: &str, client: &reqwest::Client) -> Result<reqwest::Response> {
    let version_manifest: VersionManifest = client
        .get("https://piston-meta.mojang.com/mc/game/version_manifest.json")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let mut target_version = version;

    if target_version == "latest" {
        target_version = &version_manifest.latest.release;
    }

    if target_version == "latest-snapshot" {
        target_version = &version_manifest.latest.snapshot;
    }

    let verdata = version_manifest
        .versions
        .iter()
        .find(|&v| v.id == target_version);

    let Some(verdata) = verdata else {
        bail!("Can't find the server jar for version {target_version}")
    };

    let package_manifest: PackageManifest = client
        .get(&verdata.url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let res = client
        .get(package_manifest.downloads.server.url)
        .send()
        .await?
        .error_for_status()?;

    Ok(res)
}
