use std::path::Path;

use anyhow::Result;

use crate::api::step::{Step, StepResult};

mod cache_check;
mod download;

use super::App;

impl App {
    pub async fn execute_steps(&self, dir: &Path, steps: &[Step]) -> Result<()> {
        for step in steps {
            let res = self.execute_step(dir, step).await?;

            if res == StepResult::Skip {
                break;
            }
        }

        Ok(())
    }

    pub async fn execute_step(&self, dir: &Path, step: &Step) -> Result<StepResult> {
        match step {
            Step::CacheCheck(metadata) => self.execute_step_cache_check(dir, metadata).await,

            Step::Download { url, metadata } => {
                self.execute_step_download(dir, url, metadata).await
            }

            Step::ExecuteJava { .. } => Ok(StepResult::Continue),
        }
    }
}
