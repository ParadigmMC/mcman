use anyhow::Result;

use crate::api::{app::App, models::Environment, step::{CacheLocation, FileMeta, Step}};

mod models;
pub use models::*;

pub const FABRIC_META_URL: &str = "https://meta.fabricmc.net";

pub struct FabricAPI<'a>(pub &'a App);

impl<'a> FabricAPI<'a> {
    pub async fn fetch_loaders(&self) -> Result<Vec<FabricLoader>> {
        self.0.http_get_json(format!("{FABRIC_META_URL}/v2/versions/loader")).await
    }

    pub async fn fetch_versions(&self) -> Result<Vec<FabricVersion>> {
        self.0.http_get_json(format!("{FABRIC_META_URL}/v2/versions/game")).await
    }

    pub async fn fetch_installers(&self) -> Result<Vec<FabricInstaller>> {
        self.0.http_get_json(format!("{FABRIC_META_URL}/v2/versions/installer")).await
    }

    pub async fn resolve_steps(
        &self,
        mc_version: &str,
        loader: &str,
        installer: &str,
        env: &Environment,
    ) -> Result<Vec<Step>> {
        let mut steps = vec![];

        if env.server() {
            let metadata = FileMeta {
                filename: String::from("server.jar"),
                cache: Some(CacheLocation("fabric".into(), format!("fabric-server-{mc_version}-{installer}-{loader}.jar"))),
                ..Default::default()
            };

            let url = format!(
                "{FABRIC_META_URL}/v2/versions/loader/{mc_version}/{loader}/{installer}/server/jar",
            );

            steps.push(Step::CacheCheck(metadata.clone()));
            steps.push(Step::Download { url, metadata });
        }

        if env.client() {
            let metadata = FileMeta {
                filename: String::from("client.jar"),
                cache: Some(CacheLocation("fabric".into(), format!("fabric-client-{mc_version}-{installer}-{loader}.jar"))),
                ..Default::default()
            };

            let url = format!(
                "{FABRIC_META_URL}/v2/versions/loader/{mc_version}/{loader}/{installer}/client/jar",
            );

            steps.push(Step::CacheCheck(metadata.clone()));
            steps.push(Step::Download { url, metadata });
        }

        Ok(steps)
    }
}

