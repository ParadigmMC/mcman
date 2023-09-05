use anyhow::{anyhow, Result, Context};
use mcapi::hangar::{VersionsFilter, ProjectVersion, Platform};
use cached::{proc_macro::cached, UnboundCache};

#[cached(
    type = "UnboundCache<String, ProjectVersion>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{id};{version};{mcver};{filter:?}") }"#,
    sync_writes = true,
    result = true
)]
pub async fn fetch_hangar_version(
    http_client: &reqwest::Client,
    id: &str,
    version: &str,
    mcver: &str,
    filter: VersionsFilter,
) -> Result<ProjectVersion> {
    let version = if version == "latest" {
        let versions = mcapi::hangar::fetch_project_versions(http_client, id, Some(filter)).await?;
        
        versions
            .result
            .iter()
            .next()
            .ok_or(anyhow!("No compatible versions for Hangar project '{id}'"))?
            .clone()
    } else if version.contains('$') {
        let versions = mcapi::hangar::fetch_project_versions(http_client, id, Some(filter)).await?;

        let version = version
            .replace("${mcver}", mcver)
            .replace("${mcversion}", mcver);

        versions
            .result
            .iter()
            .find(|v| &v.name == &version)
            .cloned()
            .or(versions
                .result
                .iter()
                .find(|v| v.name.contains(&version))
                .cloned())
            .ok_or(anyhow!(
                "No compatible versions ('{version}') for Hangar project '{id}'"
            ))?
    } else {
        mcapi::hangar::fetch_project_version(http_client, id, version).await?
    };

    Ok(version)
}

pub async fn get_hangar_url(
    http_client: &reqwest::Client,
    id: &str,
    version: &str,
    mcver: &str,
    filter: VersionsFilter,
) -> Result<String> {
    let version = fetch_hangar_version(http_client, id, version, mcver, filter.clone())
        .await.context("Fetching project version")?;

    let download = version
        .downloads
        .get(&filter.platform.unwrap_or(Platform::Paper))
        .ok_or(anyhow!(
            "Platform unsupported for Hangar project '{id}' version '{}'",
            version.name
        ))?;
    
    Ok(download.get_url())
}

pub async fn get_hangar_filename(
    http_client: &reqwest::Client,
    id: &str,
    version: &str,
    mcver: &str,
    filter: VersionsFilter,
) -> Result<String> {
    let version = fetch_hangar_version(http_client, id, version, mcver, filter.clone())
        .await.context("Fetching project version")?;

    let download = version
        .downloads
        .get(&filter.platform.unwrap_or(Platform::Paper))
        .ok_or(anyhow!(
            "Platform unsupported for Hangar project '{id}' version '{}'",
            version.name
        ))?;
    
    Ok(download.get_file_info().name)
}
