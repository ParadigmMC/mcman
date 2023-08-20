use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use cached::{proc_macro::cached, UnboundCache};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

async fn wait_ratelimit(res: reqwest::Response) -> Result<reqwest::Response> {
    let res = if let Some(h) = res.headers().get("x-ratelimit-remaining") {
        if String::from_utf8_lossy(h.as_bytes()) == "1" {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            let ratelimit_reset =
                String::from_utf8_lossy(res.headers()["x-ratelimit-reset"].as_bytes())
                    .parse::<u64>()?;
            let amount = ratelimit_reset - now;
            println!(" (!) Github ratelimit exceeded. sleeping for {amount} seconds...");
            sleep(Duration::from_secs(amount)).await;
        }
        res
    } else {
        res.error_for_status()?
    };

    Ok(res)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubRelease {
    pub tag_name: String,
    pub name: String,
    pub assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GithubAsset {
    pub url: String,
    pub name: String,
}

#[cached(
    type = "UnboundCache<String, Vec<GithubRelease>>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{repo}") }"#,
    sync_writes = true,
    result = true
)]
pub async fn fetch_github_releases(
    repo: &str,
    client: &reqwest::Client,
) -> Result<Vec<GithubRelease>> {
    let releases: Vec<GithubRelease> = wait_ratelimit(
        client
            .get("https://api.github.com/repos/".to_owned() + repo + "/releases")
            .send()
            .await?,
    )
    .await?
    .json()
    .await?;

    Ok(releases)
}

#[cached(
    type = "UnboundCache<String, GithubRelease>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{repo};{tag};{mcver}") }"#,
    sync_writes = true,
    result = true
)]
pub async fn fetch_github_release(
    client: &reqwest::Client,
    repo: &str,
    tag: &str,
    mcver: &str,
) -> Result<GithubRelease> {
    let releases = fetch_github_releases(repo, client).await?;

    let tag = tag.replace("${mcver}", mcver);
    let tag = tag.replace("${mcversion}", mcver);

    let release = match tag.as_str() {
        "latest" => releases.first(),
        id => releases.iter().find(|r| r.tag_name == id),
    }
    .ok_or(anyhow!(
        "Github release with tag '{tag}' not found on repository '{repo}'"
    ))?;

    Ok(release.clone())
}

#[cached(
    type = "UnboundCache<String, GithubAsset>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{repo};{tag};{asset};{mcver}") }"#,
    sync_writes = true,
    result = true
)]
pub async fn fetch_github_release_asset(
    repo: &str,
    tag: &str,
    asset: &str,
    mcver: &str,
    client: &reqwest::Client,
) -> Result<GithubAsset> {
    let release = fetch_github_release(client, repo, tag, mcver).await?;

    let assets = &release.assets;

    let resolved_asset = match asset {
        "" | "first" | "any" => assets.first(),
        id => {
            let id = if id.contains('$') {
                id.replace("${version}", &release.tag_name)
                    .replace("${tag}", &release.tag_name)
                    .replace("${release}", &release.tag_name)
                    .replace("${mcver}", mcver)
                    .replace("${mcversion}", mcver)
            } else {
                id.to_owned()
            };

            assets
                .iter()
                .find(|a| id == a.name)
                .or(assets.iter().find(|a| a.name.contains(&id)))
        }
    }
    .ok_or(anyhow!(
        "Github release asset with name '{asset}' on release '{}' not found",
        release.tag_name
    ))?;

    Ok(resolved_asset.clone())
}

#[cached(
    type = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{repo};{tag};{asset};{mcver}") }"#,
    sync_writes = true,
    result = true
)]
pub async fn fetch_github_release_filename(
    repo: &str,
    tag: &str,
    asset: &str,
    mcver: &str,
    client: &reqwest::Client,
) -> Result<String> {
    Ok(fetch_github_release_asset(repo, tag, asset, mcver, client)
        .await?
        .name)
}

#[cached(
    type = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{repo};{tag};{asset};{mcver}") }"#,
    sync_writes = true,
    result = true
)]
pub async fn get_github_release_url(
    repo: &str,
    tag: &str,
    asset: &str,
    mcver: &str,
    client: &reqwest::Client
) -> Result<String> {
    let fetched_tag = fetch_github_release(client, repo, tag, mcver).await?;
    let fetched_asset = fetch_github_release_asset(repo, tag, asset, mcver, client).await?;

    Ok(format!(
        "https://github.com/{repo}/releases/download/{}/{}",
        fetched_tag.tag_name, fetched_asset.name
    ))
}

#[cached(
    type = "UnboundCache<String, String>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{repo}") }"#,
    sync_writes = true,
    result = true
)]
pub async fn fetch_repo_description(client: &reqwest::Client, repo: &str) -> Result<String> {
    let desc = wait_ratelimit(
        client
            .get("https://api.github.com/repos/".to_owned() + repo)
            .send()
            .await?,
    )
    .await?
    .error_for_status()?
    .json::<serde_json::Value>()
    .await?["description"]
        .as_str()
        .unwrap_or_default()
        .to_owned();

    Ok(desc)
}
