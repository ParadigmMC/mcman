use anyhow::Result;

mod models;
pub use models::*;
use serde::de::DeserializeOwned;

use crate::api::{
    app::App,
    step::{CacheLocation, FileMeta, Step},
};

pub fn resource_id(slug: &str) -> &str {
    if let Some(i) = slug.find('.') {
        if i < slug.len() - 1 {
            return slug.split_at(i + 1).1;
        }
    }

    slug
}

pub struct SpigotAPI<'a>(pub &'a App);

impl<'a> SpigotAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        self.0
            .http_get_json(format!("{}/{url}", self.0.options.api_urls.spiget))
            .await
    }

    pub async fn fetch_resource(&self, id: &str) -> Result<SpigotResource> {
        self.fetch_api::<SpigotResource>(&format!("resources/{}", resource_id(id)))
            .await
    }

    pub async fn fetch_versions(&self, id: &str) -> Result<Vec<SpigotVersionDetailed>> {
        self.fetch_api(&format!(
            "resources/{}/versions?size=10000&sort=-id",
            resource_id(id)
        ))
        .await
    }

    pub async fn fetch_version(&self, id: &str, version: &str) -> Result<SpigotVersionDetailed> {
        self.fetch_api(&format!("resources/{}/versions/{version}", resource_id(id)))
            .await
    }

    pub async fn resolve_steps(&self, id: &str, version: &str) -> Result<Vec<Step>> {
        let url = format!(
            "{}/resources/{}/versions/{}/download/proxy",
            self.0.options.api_urls.spiget,
            resource_id(id),
            version,
        );

        let metadata = FileMeta {
            cache: Some(CacheLocation(
                "spiget".into(),
                format!("{}/{}.jar", resource_id(id), version),
            )),
            filename: format!("spigot-{}-{}.jar", resource_id(id), version),
            ..Default::default()
        };

        Ok(vec![
            Step::CacheCheck(metadata.clone()),
            Step::Download { url, metadata },
        ])
    }

    pub async fn resolve_remove_steps(&self, id: &str, version: &str) -> Result<Vec<Step>> {
        Ok(vec![Step::RemoveFile(FileMeta::filename(format!(
            "spigot-{}-{}.jar",
            resource_id(id),
            version
        )))])
    }
}
