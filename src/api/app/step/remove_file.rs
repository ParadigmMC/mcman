use std::path::Path;

use anyhow::{anyhow, bail, Result};

use crate::api::{app::App, step::{FileMeta, StepResult}, tools::{self, java::{JavaProcess, JavaVersion}}};

impl App {
    pub(super) async fn execute_step_remove_file(
        &self,
        dir: &Path,
        metadata: &FileMeta,
    ) -> Result<StepResult> {
        

        Ok(StepResult::Continue)
    }
}
