use std::path::PathBuf;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use super::JavaVersion;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JavaInstallation {
    pub path: PathBuf,
    pub version: JavaVersion,
    pub architecture: String,
    pub vendor: String,
}

impl JavaInstallation {
    pub fn get_version_from(version: &str) -> Result<JavaVersion> {
        let mut split = version.split('.');

        let str = match (split.next(), split.next()) {
            (Some("1"), Some(ver)) => ver,
            (Some(ver), _) => ver,
            _ => bail!("Invalid JRE version"),
        };

        Ok(str.parse::<u32>()?)
    }
}
