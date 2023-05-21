use anyhow::{Result, anyhow, Context};
use serde::{Deserialize, Serialize};

/* 
#[derive(Debug, Deserialize, Serialize)]
struct SpigotResourceFile {
    
}

// this api is so weird
#[derive(Debug, Deserialize, Serialize)]
struct SpigotVersionData {
    pub id: String,
}*/

#[derive(Debug, Deserialize, Serialize)]
struct SpigotResourceVersion {
    pub name: String,
    pub id: i32,
} 

pub fn get_resource_id(res: &str) -> Result<&str> {
    if res.contains('.') {
        let sp: Vec<&str> = res.split('.').collect();
        Ok(sp.get(1).ok_or(anyhow!("how even"))?)
    } else {
        Ok(res)
    }
}

pub async fn fetch_spigot_resource_latest_ver(
    id: &str,
    client: &reqwest::Client,
) -> Result<String> {
    let project: SpigotResourceVersion = client
        .get("https://api.spiget.org/v2/resources/".to_owned()
            + get_resource_id(id)? + "/versions/latest")
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
    let id_parsed = get_resource_id(id)?;

    Ok(client
        .get(format!("https://api.spiget.org/v2/resources/{id_parsed}/download"))
        .send()
        .await?
        .error_for_status()?)
}
