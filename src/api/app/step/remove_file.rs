use std::path::Path;
use tokio::fs;

use anyhow::Result;

use crate::api::{
    app::App,
    step::{FileMeta, StepResult},
};

impl App {
    pub(super) async fn execute_step_remove_file(
        &self,
        dir: &Path,
        metadata: &FileMeta,
    ) -> Result<StepResult> {
        println!("Deleting {}", metadata.filename);

        let path = dir.join(&metadata.filename);

        if path.exists() {
            fs::remove_file(path).await?;
        } else {
            println!("{path:?} does not exist, cant delete");
        }

        Ok(StepResult::Continue)
    }
}
