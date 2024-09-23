use std::path::Path;

use anyhow::{Context, Result};

use crate::api::step::{Step, StepResult};

mod cache_check;
mod download;
mod execute_java;
mod remove_file;

use super::App;

impl App {
    /// Execute a list of steps, taking care of their `StepResult`'s.
    /// Skips the next step when a step returns `StepResult::Skip`
    pub async fn execute_steps(&self, dir: &Path, steps: &[Step]) -> Result<()> {
        let mut iter = steps.iter();

        while let Some(step) = iter.next() {
            let res = self
                .execute_step(dir, step)
                .await
                .with_context(|| format!("Executing steps: {steps:#?}"))?;
            if res == StepResult::Skip {
                _ = iter.next();
            }
        }

        Ok(())
    }

    /// Execute a single step and return its result
    pub async fn execute_step(&self, dir: &Path, step: &Step) -> Result<StepResult> {
        match step {
            Step::CacheCheck(metadata) => self
                .execute_step_cache_check(dir, metadata)
                .await
                .with_context(|| format!("Checking for cache for {metadata:?}")),

            Step::Download { url, metadata } => self
                .execute_step_download(dir, url, metadata)
                .await
                .with_context(|| format!("URL: {url}"))
                .with_context(|| format!("File: {metadata:?}"))
                .with_context(|| "Downloading a file".to_string()),

            Step::ExecuteJava {
                args,
                java_version,
                label,
            } => {
                self.execute_step_execute_java(dir, args, *java_version, label)
                    .await
            },

            Step::RemoveFile(metadata) => self.execute_step_remove_file(dir, metadata).await,
        }
    }
}
