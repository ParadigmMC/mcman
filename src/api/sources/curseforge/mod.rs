mod models;
use anyhow::Result;
pub use models::*;
use serde::de::DeserializeOwned;

use crate::api::app::App;

pub const CURSEFORGE_API_URL: &str = "https://api.curse.tools/v1/cf";

pub struct CurseforgeAPI<'a>(pub &'a App);

impl<'a> CurseforgeAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        self.0.http_get_json(format!("{CURSEFORGE_API_URL}/{url}")).await
    }
}


