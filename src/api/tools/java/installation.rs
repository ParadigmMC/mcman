use std::path::PathBuf;

use anyhow::{bail, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;

use super::{check_java, find, JavaVersion};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JavaInstallation {
    pub path: PathBuf,
    pub version: JavaVersion,
    pub architecture: String,
    pub vendor: String,
}

impl JavaInstallation {
    pub fn parse_version(version: &str) -> Result<JavaVersion> {
        let mut split = version.split('.');

        let ((Some("1"), Some(str)) | (Some(str), _)) = (split.next(), split.next()) else {
            bail!("Invalid JRE version");
        };

        Ok(str.parse::<u32>()?)
    }
}

static JAVA_INSTALLATIONS: OnceCell<Vec<JavaInstallation>> = OnceCell::const_new();

pub async fn get_java_installations() -> &'static Vec<JavaInstallation> {
    JAVA_INSTALLATIONS
        .get_or_init(|| async {
            let paths = find::collect_possible_java_paths();

            futures::stream::iter(paths)
                .filter_map(|path| async move { check_java(&path) })
                .collect()
                .await
        })
        .await
}

pub async fn get_java_installation_for(ver: JavaVersion) -> Option<JavaInstallation> {
    get_java_installations()
        .await
        .iter()
        .find(|v| v.version == ver)
        .cloned()
}
