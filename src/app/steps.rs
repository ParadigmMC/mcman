use std::path::PathBuf;

use anyhow::Result;
use indicatif::ProgressBar;

use super::{ResolvedFile, App};

#[derive(Debug, Clone)]
pub enum Step {
    Download(DownloadTask),
    ExecuteJava {
        jar: String,
        args: Vec<String>,
        path: PathBuf,
    }
}

#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub resolved_file: ResolvedFile,
    pub path: PathBuf,
}

impl App {
    pub async fn run_step(&self, step: &Step) -> Result<()> {
        match step {
            Step::Download(folder, resolved_file) => {
                self.download_resolved(
                    resolved_file.clone(),
                    folder.clone(),
                    self.multi_progress.add(ProgressBar::new_spinner())
                ).await?;
            }

            Step::ExecuteJava { jar, args, path } => {
                todo!();
            }
        }
        
        Ok(())
    }
}
