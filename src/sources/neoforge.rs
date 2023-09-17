use anyhow::{anyhow, Context, Result};

use crate::{util, App, FileSource};

pub static NEOFORGE_MAVEN: &str = "https://maven.neoforged.net/releases";
pub static NEOFORGE_GROUP: &str = "net.neoforged";
pub static NEOFORGE_ARTIFACT: &str = "forge";
pub static NEOFORGE_FILENAME: &str = "${artifact}-${version}-installer.jar";

pub struct NeoforgeAPI<'a>(pub &'a App);

impl<'a> NeoforgeAPI<'a> {
    pub async fn fetch_versions(&self) -> Result<Vec<String>> {
        let (_, versions) = self.0.maven().fetch_versions(NEOFORGE_MAVEN, NEOFORGE_GROUP, NEOFORGE_ARTIFACT).await?;

        Ok(versions
            .iter()
            .filter_map(|s| {
                let (m, l) = s.split_once('-')?;
    
                if m == self.0.mc_version() {
                    Some(l.to_owned())
                } else {
                    None
                }
            })
            .collect())
    }

    pub async fn fetch_latest(&self) -> Result<String> {
        util::get_latest_semver(&self.fetch_versions().await?).ok_or(anyhow!("No forge loader versions for {}", self.0.mc_version()))
    }

    pub async fn resolve_version(
        &self, loader: &str,
    ) -> Result<String> {
        Ok(if loader == "latest" || loader.is_empty() {
            self.fetch_latest()
                .await
                .context("Getting latest Forge version")?
        } else {
            loader.to_owned()
        })
    }

    pub async fn resolve_source(&self, loader: &str) -> Result<FileSource> {
        self.0.maven().resolve_source(NEOFORGE_MAVEN, NEOFORGE_GROUP, NEOFORGE_ARTIFACT, &format!(
            "{}-{}",
            self.0.mc_version(),
            self.resolve_version(loader).await?
        ),
        NEOFORGE_FILENAME).await
    }
}
