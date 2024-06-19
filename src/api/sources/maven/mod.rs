use anyhow::{anyhow, Result};

mod models;
pub use models::*;

use crate::api::{app::App, step::{CacheLocation, FileMeta, Step}, utils::url::url_to_folder};

pub struct MavenAPI<'a>(pub &'a App);

pub fn maven_metadata_url(url: &str, group_id: &str, artifact_id: &str) -> String {
    format!(
        "{url}/{}/{artifact_id}/maven-metadata.xml",
        group_id.replace(['.', ':'], "/")
    )
}

// @author ChatGPT
pub fn guess_maven_metadata_url(url: &str) -> Result<String> {
    // Attempt to construct the Maven metadata URL based on the provided URL
    let segments: Vec<&str> = url.trim_end_matches('/').rsplit('/').collect();

    if let Some(last_segment) = segments.first() {
        if last_segment.is_empty() {
            // If the last segment is empty, skip it
            let metadata_url = format!("{}/maven-metadata.xml", url.trim_end_matches('/'));
            return Ok(metadata_url);
        }
    }

    if segments.len() >= 2 {
        // Construct the Maven metadata URL by going up one level
        let metadata_url = format!(
            "{}/maven-metadata.xml",
            url.trim_end_matches(segments[0]).trim_end_matches('/')
        );
        Ok(metadata_url)
    } else {
        Err(anyhow!("Invalid URL format"))
    }
}

impl<'a> MavenAPI<'a> {
    pub async fn find_maven_artifact(&self, url: &str) -> Result<MavenMetadata> {
        let metadata_url = if url.ends_with("/maven-metadata.xml") {
            url.to_string()
        } else {
            guess_maven_metadata_url(url)?
        };

        self.fetch_metadata_url(&metadata_url).await
    }

    pub async fn fetch_metadata(
        &self,
        url: &str,
        group_id: &str,
        artifact_id: &str,
    ) -> Result<MavenMetadata> {
        self.fetch_metadata_url(&maven_metadata_url(url, group_id, artifact_id))
            .await
    }

    pub async fn fetch_metadata_url(&self, url: &str) -> Result<MavenMetadata> {
        let xml = self.0.http_client.get(url).send().await?.text().await?;

        let doc = roxmltree::Document::parse(&xml)?;

        Ok(MavenMetadata {
            latest: doc.get_text("latest").ok(),
            artifact_id: doc.get_text("artifactId").ok(),
            group_id: doc.get_text("groupId").ok(),
            versions: doc.get_text_all("version"),
        })
    }

    pub async fn resolve_steps(
        &self,
        url: &str,
        group_id: &str,
        artifact_id: &str,
        version: &str,
        file: &str,
    ) -> Result<Vec<Step>> {
        let file = file
            .replace("${artifact}", artifact_id)
            .replace("${version}", version);

        let file = if file.contains('.') {
            file
        } else {
            format!("{file}.jar")
        };

        let download_url = format!(
            "{url}/{}/{artifact_id}/{version}/{file}",
            group_id.replace('.', "/"),
        );

        let metadata = FileMeta {
            cache: Some(CacheLocation("maven".into(), format!(
                "{}/{}/{artifact_id}/{version}/{file}",
                url_to_folder(url),
                group_id.replace('.', "/"),
            ))),
            filename: file,
            ..Default::default()
        };

        Ok(vec![
            Step::CacheCheck(metadata.clone()),
            Step::Download { url: download_url, metadata },
        ])
    }
}
