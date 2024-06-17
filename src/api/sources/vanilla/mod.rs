use anyhow::{anyhow, bail, Result};

use crate::api::{app::App, models::Environment, step::Step};

mod assets;
mod manifest;
mod version;
mod rulematcher;

pub use self::{assets::*, manifest::*, version::*, rulematcher::*};

pub struct VanillaAPI<'a>(pub &'a App);

impl<'a> VanillaAPI<'a> {
    pub async fn fetch_manifest(&self) -> Result<VersionManifest> {
        self.0.http_get_json(VERSION_MANIFEST_URL).await
    }

    pub async fn fetch_latest_mcver(&self) -> Result<String> {
        Ok(self.fetch_manifest()
            .await?
            .latest
            .release)
    }

    pub async fn resolve_steps(&self, version: &str, env: Environment) -> Result<Vec<Step>> {
        let env = match env {
            Environment::Client => DownloadType::Client,
            Environment::Server => DownloadType::Server,
            _ => bail!("You cant have both smh"),
        };

        let manifest = self.fetch_manifest().await?;

        let indexed = manifest.versions.into_iter().find(|v| v.id == version).ok_or(anyhow!("Cant find version '{version}'"))?;

        let version: VersionInfo = self.0.http_get_json(indexed.url).await?;

        let steps = version.into_step(env).ok_or(anyhow!("Cant find download"))?;

        Ok(steps)
    }
}
