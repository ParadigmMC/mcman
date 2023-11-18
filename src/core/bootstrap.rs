use std::{path::{Path, PathBuf}, collections::HashMap, time::{SystemTime, Duration}};

use anyhow::{Result, Context, anyhow};
use indicatif::{ProgressBar, ProgressStyle};
use pathdiff::diff_paths;
use tokio::fs;
use walkdir::WalkDir;

use crate::model::BootstrappedFile;

use super::BuildContext;

impl<'a> BuildContext<'a> {
    pub async fn bootstrap_files(&mut self) -> Result<()> {
        self.app.print_job("Bootstrapping...");

        self.app.ci("::group::Bootstrapping");

        let pb = self.app.multi_progress.add(ProgressBar::new_spinner()
            .with_style(ProgressStyle::with_template("{spinner:.blue} {prefix} {msg}")?)
            .with_prefix("Bootstrapping"));
        pb.enable_steady_tick(Duration::from_millis(250));

        let lockfile_entries: HashMap<PathBuf, SystemTime> = HashMap::from_iter(self.lockfile.files.iter()
            .map(|e| (e.path.clone(), e.date.clone())));

        for entry in WalkDir::new(self.app.server.path.join("config")) {
            let entry = entry
                .map_err(|e| anyhow!(
                    "Can't walk directory/file: {}",
                    &e.path().unwrap_or(Path::new("<unknown>")
                ).display()))?;
    
            if entry.file_type().is_dir() {
                continue;
            }

            let source = entry.path();
            let diffed_paths = diff_paths(&source, self.app.server.path.join("config"))
                .ok_or(anyhow!("Cannot diff paths"))?;

            pb.set_message(diffed_paths.to_string_lossy().to_string());
    
            self.bootstrap_file(&diffed_paths, lockfile_entries.get(&diffed_paths)).await.context(format!(
                "Bootstrapping file:
                - Entry: {}
                - Relative: {}",
                entry.path().display(),
                diffed_paths.display()
            ))?;
        }

        pb.disable_steady_tick();
        pb.finish_and_clear();
        self.app.success("Bootstrapping complete");

        self.app.ci("::endgroup::");

        Ok(())
    }

    pub fn should_bootstrap_file(&self, path: &Path) -> bool {
        let ext = path.extension().unwrap_or_default().to_str().unwrap_or_default();

        let bootstrap_exts = vec![
            "properties", "txt", "yaml", "yml", "conf", "config", "toml", "json", "json5", "secret"
        ];

        bootstrap_exts.contains(&ext) || self.app.server.options.bootstrap_exts.contains(&ext.to_string())
    }

    pub async fn bootstrap_file(
        &mut self,
        rel_path: &PathBuf,
        cache: Option<&SystemTime>
    ) -> Result<()> {
        let pretty_path = rel_path.display();

        let source = self.app.server.path.join("config").join(rel_path);
        let dest = self.output_dir.join(rel_path);

        let metadata = fs::metadata(&source).await
            .context(format!(
                "Getting metadata
                - File: {}
                - Relative path: {pretty_path}
                - Destination: {}",
                source.display(),
                dest.display(),
            ))?;

        let modified = metadata.modified();

        if self.force || {
            if let Some(time) = cache {
                if let Ok(source_time) = modified {
                    &source_time > time
                } else { true }
            } else {
                true
            }
        } {
            fs::create_dir_all(dest.parent().unwrap()).await
                .context("Creating parent directory")?;

            if self.should_bootstrap_file(rel_path) {
                let config_contents = fs::read_to_string(&source)
                    .await.context(format!("Reading from '{}' ; [{pretty_path}]", source.display()))?;
    
                let bootstrapped_contents = self.bootstrap_content(&config_contents);
    
                fs::write(&dest, bootstrapped_contents)
                    .await.context(format!("Writing to '{}' ; [{pretty_path}]", dest.display()))?;
            } else {
                // ? idk why but read_to_string and fs::write works with 'dest' but fs::copy doesnt
                fs::copy(&source, &dest)
                    .await.context(format!("Copying '{}' to '{}' ; [{pretty_path}]", source.display(), dest.display()))?;
            }
    
            self.app.log(format!("  -> {pretty_path}"));
        } else {
            self.app.log(format!("  unchanged: {pretty_path}"));
        }

        if let Ok(source_time) = modified {
            self.new_lockfile.files.push(BootstrappedFile {
                path: rel_path.clone(),
                date: source_time
            });
        } else {
            self.app.warn("File metadata not supported");
        }

        Ok(())
    }

    pub fn bootstrap_content(&self, content: &str) -> String {
        mcapi::dollar_repl(content, |k| {
            let k = k.trim();

            let (k, def) = if let Some((k, def)) = k.split_once(':') {
                (k.trim(), Some(def.trim().to_owned()))
            } else {
                (k, None)
            };

            self.app.var(k).or(def)
        })
    }
}
