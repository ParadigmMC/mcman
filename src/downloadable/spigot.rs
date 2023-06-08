use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct SpigotResourceVersion {
    pub name: String,
    pub id: i32,
}

pub fn get_resource_id(res: &str) -> &str {
    if let Some(i) = res.find('.') {
        if i < res.len() - 1 {
            return res.split_at(i + 1).1;
        }
    }

    res
}

pub async fn fetch_spigot_resource_latest_ver(
    id: &str,
    client: &reqwest::Client,
) -> Result<String> {
    let project: SpigotResourceVersion = client
        .get(
            "https://api.spiget.org/v2/resources/".to_owned()
                + get_resource_id(id)
                + "/versions/latest",
        )
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(project.id.to_string())
}

pub async fn download_spigot_resource(
    id: &str,
    client: &reqwest::Client,
) -> Result<reqwest::Response> {
    //let version = fetch_spigot_resource_latest_ver(id, client).await.context("fetching latest version")?;
    let id_parsed = get_resource_id(id);

    Ok(client
        .get(format!(
            "https://api.spiget.org/v2/resources/{id_parsed}/download"
        ))
        .send()
        .await?
        .error_for_status()?)
}
