pub const JAVA_BIN: &str = "java";
pub type JavaVersion = u32;

mod installation;
mod find;
mod check;
use std::{path::Path, process::{ExitStatus, Stdio}};

use anyhow::{anyhow, Result};
pub use installation::*;
pub use check::*;
use tokio::{io::{AsyncBufReadExt, BufReader}, process::{Child, Command}};

use crate::api::utils::pathdiff::DiffTo;

pub struct JavaProcess {
    child: Child,
}

impl JavaProcess {
    pub fn new(
        dir: &Path,
        java: &Path,
        args: &[&str],
    ) -> Result<Self> {
        // JRE is stupid
        let dir = std::env::current_dir()?
            .diff_to(dir)
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

    pub fn lines<F>(&mut self, f: F) -> ()
    where
        F: Fn(&str) + Send + 'static
    {
        let stdout = self.child.stdout
            .take()
            .expect("Child to have stdout");

        let mut lines = BufReader::new(stdout).lines();
        
        tokio::spawn(async move {
            while let Ok(Some(line)) = lines.next_line().await {
                f(line.trim());
            }
        });
    }

    pub async fn wait(&mut self) -> Result<ExitStatus> {
        Ok(self.child.wait().await?)
    }
}
