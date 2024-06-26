use std::path::Path;

use anyhow::{Context, Result};

use crate::api::step::{Step, StepResult};

mod cache_check;
mod download;
mod execute_java;

use super::App;

impl App {
    pub async fn execute_steps(&self, dir: &Path, steps: &[Step]) -> Result<()> {
        let mut iter = steps.iter();

        while let Some(step) = iter.next() {
            let res = self.execute_step(dir, step).await?;
            if res == StepResult::Skip {
                _ = iter.next();
            }
        }

        Ok(())
    }

    pub async fn execute_step(&self, dir: &Path, step: &Step) -> Result<StepResult> {
        match step {
            Step::CacheCheck(metadata) => self.execute_step_cache_check(dir, metadata).await
                .with_context(|| format!("Checking for cache for {metadata:?}")),

            Step::Download { url, metadata } => {
                self.execute_step_download(dir, url, metadata).await
                    .with_context(|| format!("URL: {url}"))
                    .with_context(|| format!("File: {metadata:?}"))
                    .with_context(|| format!("Downloading a file"))
            }

            Step::ExecuteJava {
                args,
                java_version,
                label
            } => {
                self.execute_step_execute_java(dir, args, *java_version, label).await
            },
        }
    }
}
