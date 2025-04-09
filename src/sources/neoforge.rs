use anyhow::{anyhow, Context, Result};

use crate::{app::App, app::ResolvedFile, util};

pub static NEOFORGE_MAVEN: &str = "https://maven.neoforged.net/releases";
pub static NEOFORGE_GROUP: &str = "net.neoforged";
pub static NEOFORGE_ARTIFACT: &str = "neoforge";
pub static NEOFORGE_FILENAME: &str = "${artifact}-${version}-installer.jar";

pub struct NeoforgeAPI<'a>(pub &'a App);

impl<'a> NeoforgeAPI<'a> {
    pub async fn fetch_versions(&self) -> Result<Vec<String>> {
        let (_, versions) = self
            .0
            .maven()
            .fetch_versions(NEOFORGE_MAVEN, NEOFORGE_GROUP, NEOFORGE_ARTIFACT)
            .await?;

        let mc_version = self.0.mc_version();
        let trimmed = mc_version.strip_prefix("1.").unwrap_or(mc_version);

        let mut candidates = versions
            .iter()
            .filter(|v| !v.contains("beta"))
            .filter(|v| v.starts_with(trimmed))
            .cloned()
            .collect::<Vec<_>>();

        candidates.sort_by(|a, b| Self::version_key(b).cmp(&Self::version_key(a)));

        Ok(candidates.into_iter().take(1).collect())
    }

    fn version_key(v: &str) -> Vec<u32> {
        v.split('.').filter_map(|s| s.parse::<u32>().ok()).collect()
    }

    pub async fn fetch_latest(&self) -> Result<String> {
        util::get_latest_semver(&self.fetch_versions().await?).ok_or(anyhow!(
            "No forge loader versions for {}",
            self.0.mc_version()
        ))
    }

    pub async fn resolve_version(&self, loader: &str) -> Result<String> {
        Ok(if loader == "latest" || loader.is_empty() {
            self.fetch_latest()
                .await
                .context("Getting latest Forge version")?
        } else {
            loader.to_owned()
        })
    }

    pub async fn resolve_source(&self, loader: &str) -> Result<ResolvedFile> {
        self.0
            .maven()
            .resolve_source(
                NEOFORGE_MAVEN,
                NEOFORGE_GROUP,
                NEOFORGE_ARTIFACT,
                &format!("{}", self.resolve_version(loader).await?),
                NEOFORGE_FILENAME,
            )
            .await
    }
}
