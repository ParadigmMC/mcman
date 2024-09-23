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
            .into_iter()
            .find(|j| j.version >= version.unwrap_or_default())
            .ok_or(anyhow!(
                "Java with version {} or higher not found, cannot proceed",
                version.map(|v| v.to_string()).unwrap_or("any".to_owned())
            ))?;

        let mut proc = JavaProcess::new(&dir.canonicalize()?, &java.path, args)?;

        fn on_line(line: &str) {
            println!("| {line}");
        }

        proc.lines(on_line);

        let res = proc.wait().await?;

        if !res.success() {
            bail!(
                "Java process exited with code {}",
                res.code()
                    .map(|x| x.to_string())
                    .unwrap_or("unknown".to_owned())
            );
        }

        Ok(StepResult::Continue)
    }
}
