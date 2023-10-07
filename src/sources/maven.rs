use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::app::{App, ResolvedFile, CacheStrategy};

pub struct MavenAPI<'a>(pub &'a App);

impl<'a> MavenAPI<'a> {
    pub fn get_metadata_url(url: &str, group_id: &str, artifact_id: &str) -> String {
        format!(
            "{url}/{}/{artifact_id}/maven-metadata.xml",
            group_id.replace('.', "/")
        )
    }

    pub async fn fetch_versions(
        &self,
        url: &str,
        group_id: &str,
        artifact_id: &str,
    ) -> Result<(String, Vec<String>)> {
        let xml = self.0.http_client
            .get(Self::get_metadata_url(url, group_id, artifact_id))
            .send()
            .await?
            .text()
            .await?;

        let doc = roxmltree::Document::parse(&xml)?;

        let latest = doc.descendants().find_map(|t| {
            if t.tag_name().name() == "latest" {
                Some(t.text()?.to_owned())
            } else {
                None
            }
        });

        let list = doc
            .descendants()
            .filter_map(|t| {
                if t.tag_name().name() == "version" {
                    Some(t.text()?.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok((
            latest.unwrap_or_else(|| list.first().cloned().unwrap_or_default()),
            list,
        ))
    }

    pub async fn fetch_version(
        &self,
        url: &str,
        group_id: &str,
        artifact_id: &str,
        version: &str,
    ) -> Result<String> {
        let fetch_versions = || self.fetch_versions(url, group_id, artifact_id);

        let version = match version {
            "latest" => fetch_versions().await?.0,
            id => {
                if id.contains('$') {
                    let versions = fetch_versions().await?.1;
                    let id = id
                        .replace("${artifact}", artifact_id)
                        .replace("${mcversion}", &self.0.mc_version())
                        .replace("${mcver}", &self.0.mc_version());
                    versions
                        .iter()
                        .find(|v| *v == &id)
                        .or_else(|| versions.iter().find(|v| v.contains(&id)))
                        .ok_or(anyhow!("Couldn't resolve maven artifact version (url={url},g={group_id},a={artifact_id})"))?
                        .clone()
                } else {
                    id.to_owned()
                }
            }
        };
    
        Ok(version)
    }

    pub async fn resolve_source(
        &self,
        url: &str,
        group_id: &str,
        artifact_id: &str,
        version: &str,
        file: &str,
    ) -> Result<ResolvedFile> {
        let version = self.fetch_version(url, group_id, artifact_id, version).await?;

        let file = file
            .replace("${artifact}", artifact_id)
            .replace("${version}", &version)
            .replace("${mcversion}", &self.0.mc_version())
            .replace("${mcver}", &self.0.mc_version());

        let file = if file.contains('.') {
            file
        } else {
            file + ".jar"
        };

        let download_url = format!(
            "{url}/{}/{artifact_id}/{version}/{file}",
            group_id.replace('.', "/"),
        );

        let cached_file_path = format!(
            "{}/{}/{artifact_id}/{version}/{file}",
            crate::util::url_to_folder(url),
            group_id.replace(".", "/"),
        );

        Ok(ResolvedFile {
            url: download_url,
            filename: file,
            cache: CacheStrategy::File { 
                namespace: String::from("maven"),
                path: cached_file_path },
            size: None,
            hashes: HashMap::new(),
        })
    }
}
