use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpigotResource {
    pub id: i32,
    pub name: String,
    pub tag: String,
    pub contributors: String,
    pub likes: i32,
    pub tested_versions: Vec<String>,
    pub versions: Vec<SpigotVersion>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpigotVersion {
    pub id: i32,
    pub uuid: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpigotVersionDetailed {
    pub id: i32,
    pub uuid: String,
    pub name: String,
}
