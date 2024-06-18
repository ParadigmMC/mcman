use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};

use crate::api::{
    app::App,
    models::Environment,
    step::{CacheLocation, FileMeta, Step},
    utils::hashing::HashFormat,
};

mod assets;
mod manifest;
mod rulematcher;
mod version;

pub use self::{assets::*, manifest::*, rulematcher::*, version::*};

impl VersionInfo {
    pub fn into_step(&self, ty: DownloadType) -> Option<Vec<Step>> {
        let file = self.downloads.get(&ty)?;

        let filename = format!("{}-{ty:?}.jar", self.id);

        let metadata = FileMeta {
            filename: filename.clone(),
            cache: Some(CacheLocation("pistonmeta".into(), filename)),
            size: Some(file.size),
            hashes: HashMap::from([(HashFormat::Sha1, file.sha1.clone())]),
        };

        Some(vec![
            Step::CacheCheck(metadata.clone()),
            Step::Download {
                url: file.url.clone(),
                metadata,
            },
        ])
    }
}

pub struct VanillaAPI<'a>(pub &'a App);

impl<'a> VanillaAPI<'a> {
    pub async fn fetch_manifest(&self) -> Result<VersionManifest> {
        self.0.http_get_json(VERSION_MANIFEST_URL).await
    }

    pub async fn fetch_latest_mcver(&self) -> Result<String> {
        Ok(self.fetch_manifest().await?.latest.release)
    }

    pub async fn resolve_steps(&self, version: &str, env: Environment) -> Result<Vec<Step>> {
        let env = match env {
            Environment::Client => DownloadType::Client,
            Environment::Server => DownloadType::Server,
            _ => bail!("You cant have both smh"),
        };

        let manifest = self.fetch_manifest().await?;

        let indexed = manifest
            .versions
            .into_iter()
            .find(|v| v.id == version)
            .ok_or(anyhow!("Cant find version '{version}'"))?;

        let version: VersionInfo = self.0.http_get_json(indexed.url).await?;

        let steps = version
            .into_step(env)
            .ok_or(anyhow!("Cant find download"))?;

        Ok(steps)
    }
}
