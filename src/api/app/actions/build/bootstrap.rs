use std::{collections::HashSet, path::Path, sync::Arc};

use anyhow::{Context, Result};
use futures::{StreamExt, TryStreamExt};
use walkdir::WalkDir;

use crate::api::{app::App, utils::{fs::create_parents, pathdiff::DiffTo}};

impl App {
    pub async fn action_bootstrap(self: Arc<Self>, base: &Path) -> Result<()> {
        let mut list = vec![];

        if let Some(path) = self.server.read().await.as_ref().map(|(path, _)| path.clone()) {
            list.push(path.parent().unwrap().join("config"));
        }

        // TODO: use self.network

        for entry in list {
            self.clone().action_bootstrap_recursive(base, &entry).await?;
        }

        Ok(())
    }

    pub async fn action_bootstrap_recursive(self: Arc<Self>, output_base: &Path, input_base: &Path) -> Result<()> {
        let output_base = Arc::new(output_base);
        let input_base = Arc::new(input_base);
        
        if !input_base.exists() {
            println!("{input_base:?} doesnt exist");
            return Ok(());
        }

        const MAX_CONCURRENT_TASKS: usize = 20;

        let x = input_base.clone();
        futures::stream::iter(WalkDir::new(*input_base.clone())).map(|res| res.with_context({
            || format!("Bootstrapping folder: {x:?}")
        })).try_for_each_concurrent(
            Some(MAX_CONCURRENT_TASKS),
            move |entry| {
                let app = self.clone();
                let output_base = output_base.clone();
                let input_base = input_base.clone();
                async move {
                    if entry.file_type().is_dir() {
                        return Ok(());
                    }

                    app.action_bootstrap_file(&output_base, &input_base, entry.path())
                        .await
                        .with_context(|| format!("Bootstrapping file: {:?}", entry.path()))
                }
            }
        ).await?;

        Ok(())
    }

    pub async fn should_bootstrap_file(&self, file: &Path) -> bool {
        let ext = file.extension().unwrap_or_default().to_str().unwrap_or_default();

        let bootstrap_exts = [
            "properties",
            "txt",
            "yaml",
            "yml",
            "conf",
            "config",
            "toml",
            "json",
            "json5",
            "secret",
        ];

        bootstrap_exts.contains(&ext)
    }

    pub async fn action_bootstrap_file(self: Arc<Self>, output_base: &Path, input_base: &Path, file: &Path) -> Result<()> {
        let lockfile_entry = self.existing_lockfile.read().await.as_ref().map(|lock| lock.bootstrapped_files.get(file)).flatten().cloned();

        // TODO: should cancel?

        let relative = input_base.try_diff_to(file)?;

        let output_path = output_base.join(&relative);

        if self.should_bootstrap_file(file).await {
            let original_contents = tokio::fs::read_to_string(file).await
                .with_context(|| format!("Reading contents of {file:?}"))?;

            let (bootstrapped_contents, _used_vars) = self.vars_replace_content(&original_contents).await?;

            create_parents(&output_path).await?;
            tokio::fs::write(&output_path, bootstrapped_contents.as_ref()).await
                .with_context(|| format!("Writing to {output_path:?}"))?;

            println!("Bootstrapped: {relative:?}");
        } else {
            create_parents(&output_path).await?;
            tokio::fs::copy(file, &output_path).await
                .with_context(|| format!("Copying {file:?} to {output_path:?}"))?;

            println!("Copied: {relative:?}");
        }
        

        // TODO: record

        Ok(())
    }
}
