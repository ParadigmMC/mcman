pub const JAVA_BIN: &str = "java";
pub type JavaVersion = u32;

mod installation;
mod find;
mod check;
use std::{path::Path, process::Stdio};

use anyhow::{anyhow, Result};
use futures::StreamExt;
pub use installation::*;
pub use check::*;
use tokio::process::{Child, Command};

use crate::api::utils::pathdiff::diff_paths;

pub struct JavaProcess {
    child: Child,
}

impl JavaProcess {
    pub fn new(
        dir: &Path,
        java: &Path,
        args: &[&str],
    ) -> Result<Self> {
        let dir = diff_paths(dir, std::env::current_dir()?)
            .ok_or(anyhow!("Couldn't diff paths"))?;

        let child = Command::new(java)
            .args(args)
            .current_dir(dir)
            .stderr(Stdio::inherit())
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()?;
        
        Ok(Self {
            child,
        })
    }
}

pub async fn get_java_installations() -> Vec<JavaInstallation> {
    let paths = find::collect_possible_java_paths();

    futures::stream::iter(paths)
        .filter_map(|path| async move {
            check_java(&path)
        })
        .collect()
        .await
}

pub async fn get_java_installation_for(ver: JavaVersion) -> Option<JavaInstallation> {
    get_java_installations()
        .await
        .into_iter()
        .find(|v| v.version == ver)
}
