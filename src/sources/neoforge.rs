use anyhow::{anyhow, Context, Result};

use crate::util;

use super::maven;

pub static NEOFORGE_MAVEN: &str = "https://maven.neoforged.net/releases";
pub static NEOFORGE_GROUP: &str = "net.neoforged";
pub static NEOFORGE_ARTIFACT: &str = "forge";
pub static NEOFORGE_FILENAME: &str = "${artifact}-${version}-installer.jar";

pub async fn get_versions_for(mcver: &str, client: &reqwest::Client) -> Result<Vec<String>> {
    let (_, versions) =
        maven::get_maven_versions(client, NEOFORGE_MAVEN, NEOFORGE_GROUP, NEOFORGE_ARTIFACT)
            .await?;

    Ok(versions
        .iter()
        .filter_map(|s| {
            let (m, l) = s.split_once('-')?;

            if m == mcver {
                Some(l.to_owned())
            } else {
                None
            }
        })
        .collect())
}

pub async fn get_latest_version_for(mcver: &str, client: &reqwest::Client) -> Result<String> {
    let loader_versions = get_versions_for(mcver, client)
        .await
        .context("Fetching NeoForge versions")?;

    util::get_latest_semver(&loader_versions).ok_or(anyhow!("No loader versions for {mcver}"))
}

pub async fn map_neoforge_version(
    loader: &str,
    mcver: &str,
    client: &reqwest::Client,
) -> Result<String> {
    Ok(if loader == "latest" || loader.is_empty() {
        get_latest_version_for(mcver, client)
            .await
            .context("Getting latest NeoForge version")?
    } else {
        loader.to_owned()
    })
}

pub async fn get_neoforge_installer_url(
    loader: &str,
    mcver: &str,
    client: &reqwest::Client,
) -> Result<String> {
    maven::get_maven_url(
        client,
        NEOFORGE_MAVEN,
        NEOFORGE_GROUP,
        NEOFORGE_ARTIFACT,
        &format!(
            "{mcver}-{}",
            map_neoforge_version(loader, mcver, client).await?
        ),
        NEOFORGE_FILENAME,
        mcver,
    )
    .await
}

pub async fn get_neoforge_installer_filename(
    loader: &str,
    mcver: &str,
    client: &reqwest::Client,
) -> Result<String> {
    maven::get_maven_filename(
        client,
        NEOFORGE_MAVEN,
        NEOFORGE_GROUP,
        NEOFORGE_ARTIFACT,
        &format!(
            "{mcver}-{}",
            map_neoforge_version(loader, mcver, client).await?
        ),
        NEOFORGE_FILENAME,
        mcver,
    )
    .await
}
