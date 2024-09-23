use std::path::Path;

use anyhow::{anyhow, bail, Result};

use crate::api::{
    app::App,
    step::StepResult,
    tools::{
        self,
        java::{JavaProcess, JavaVersion},
    },
};

impl App {
    pub(super) async fn execute_step_execute_java(
        &self,
        dir: &Path,
        args: &Vec<String>,
        version: Option<JavaVersion>,
        label: &str,
    ) -> Result<StepResult> {
        println!("Executing java");

        let java = tools::java::get_java_installations()
            .await
            .iter()
            .find(|j| j.version >= version.unwrap_or_default())
            .ok_or(anyhow!(
                "Java with version {} or higher not found, cannot proceed",
                version.map_or("any".to_owned(), |v| v.to_string())
            ))?;

        let mut proc = JavaProcess::new(&dir.canonicalize()?, &java.path, args)?;

        proc.lines(|line| println!("| {line}"));

        let res = proc.wait().await?;

        if !res.success() {
            bail!(
                "Java process exited with code {}",
                res.code().map_or("unknown".to_owned(), |x| x.to_string())
            );
        }

        Ok(StepResult::Continue)
    }
}
