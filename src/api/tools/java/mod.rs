pub const JAVA_BIN: &str = "java";
pub type JavaVersion = u32;

mod installation;
mod find;
mod check;
use std::{ffi::OsStr, fmt::Debug, path::Path, process::{ExitStatus, Stdio}};

use anyhow::{anyhow, Context, Result};
pub use installation::*;
pub use check::*;
use tokio::{io::{AsyncBufReadExt, BufReader}, process::{Child, Command}};

use crate::api::utils::pathdiff::DiffTo;

pub struct JavaProcess {
    child: Child,
}

impl JavaProcess {
    pub fn new<I: IntoIterator<Item = S1> + Debug, S1: AsRef<OsStr> + Debug, S2: AsRef<OsStr> + Debug>(
        dir: &Path,
        java: S2,
        args: I,
    ) -> Result<Self> {
        // JRE is stupid
        let dir = if std::env::consts::OS == "windows" {
            std::env::current_dir()?
                .try_diff_to(dir)?
        } else {
            dir.to_path_buf()
        };

        log::info!("Running java process");
        log::info!("Cwd: {dir:?}");
        log::info!("Java binary: {java:#?}");
        log::info!("Arguments: {args:#?}");

        let child = Command::new(&java)
            .args(args)
            .current_dir(dir)
            .stderr(Stdio::inherit())
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .with_context(|| format!("Spawning java process"))?;
        
        log::info!("Child process spawned");
        
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

    pub async fn kill(&mut self) -> Result<()> {
        self.child.kill().await?;
        Ok(())
    }

    pub async fn wait(&mut self) -> Result<ExitStatus> {
        Ok(self.child.wait().await?)
    }
}
