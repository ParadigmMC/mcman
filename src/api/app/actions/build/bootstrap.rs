use std::{collections::HashSet, path::Path, sync::Arc};

use anyhow::{Context, Result};
use futures::{StreamExt, TryStreamExt};
use walkdir::WalkDir;

use crate::api::{
    app::App,
    utils::{fs::create_parents, pathdiff::DiffTo},
};

impl App {
    /// Collects a list of paths to boostrap from
    /// and calls [`Self::action_bootstrap_recursive`] for each folder root
    pub async fn action_bootstrap(self: Arc<Self>, base: &Path) -> Result<()> {
        let mut changed_variables = HashSet::new();

        for (k, v) in &self
            .existing_lockfile
            .read()
            .await
            .as_ref()
            .map(|lock| lock.vars.clone())
            .unwrap_or_default()
        {
            if !self
                .resolve_variable_value(k)
                .await
                .is_some_and(|value| &value == v)
            {
                changed_variables.insert(k.clone());
            }
        }

        let list = self.collect_bootstrap_paths().await;
        for entry in list {
            self.clone()
                .action_bootstrap_recursive(base, &entry, &changed_variables)
                .await?;
        }

        Ok(())
    }

    /// Recursively walks through the directory and bootstraps every eccountered file
    pub async fn action_bootstrap_recursive(
        self: Arc<Self>,
        output_base: &Path,
        input_base: &Path,
        changed_variables: &HashSet<String>,
    ) -> Result<()> {
        let output_base = Arc::new(output_base);
        let input_base = Arc::new(input_base);

        if !input_base.exists() {
            log::warn!("{input_base:?} doesnt exist");
            return Ok(());
        }

        const MAX_CONCURRENT_TASKS: usize = 20;

        let x = input_base.clone();
        futures::stream::iter(WalkDir::new(*input_base.clone()))
            .map(|res| res.with_context({ || format!("Bootstrapping folder: {x:?}") }))
            .try_for_each_concurrent(Some(MAX_CONCURRENT_TASKS), move |entry| {
                let app = self.clone();
                let output_base = output_base.clone();
                let input_base = input_base.clone();
                async move {
                    if entry.file_type().is_dir() {
                        return Ok(());
                    }

                    app.action_bootstrap_file(
                        &output_base,
                        &input_base,
                        entry.path(),
                        &changed_variables,
                    )
                    .await
                    .with_context(|| format!("Bootstrapping file: {:?}", entry.path()))
                }
            })
            .await?;

        Ok(())
    }

    /// Decides if file should be 'bootstrapped' (uses variables etc.) or not (straight up copy file)
    pub async fn should_bootstrap_file(&self, file: &Path) -> bool {
        let ext = file
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

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

    /// Process a single file. Calls [`Self::should_bootstrap_file`] on the file to decide if it should process it
    /// or copy it.
    #[allow(unused)]
    pub async fn action_bootstrap_file(
        self: Arc<Self>,
        output_base: &Path,
        input_base: &Path,
        file: &Path,
        changed_variables: &HashSet<String>,
    ) -> Result<()> {
        //let lockfile_entry = self.existing_lockfile.read().await.as_ref().map(|lock| lock.bootstrapped_files.get(file)).flatten().cloned();

        let relative = input_base.try_diff_to(file)?;

        let output_path = output_base.join(&relative);

        if self.should_bootstrap_file(file).await {
            let original_contents = tokio::fs::read_to_string(file)
                .await
                .with_context(|| format!("Reading contents of {file:?}"))?;

            let (bootstrapped_contents, _used_vars) =
                self.vars_replace_content(&original_contents).await?;

            create_parents(&output_path).await?;
            tokio::fs::write(&output_path, bootstrapped_contents.as_ref())
                .await
                .with_context(|| format!("Writing to {output_path:?}"))?;

            log::info!("Bootstrapped: {relative:?}");
        } else {
            create_parents(&output_path).await?;
            tokio::fs::copy(file, &output_path)
                .await
                .with_context(|| format!("Copying {file:?} to {output_path:?}"))?;

            log::info!("Copied: {relative:?}");
        }

        // TODO: record

        Ok(())
    }
}
