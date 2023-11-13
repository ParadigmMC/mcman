use std::{path::PathBuf, process::Child, time::Duration, fmt::Debug};

use anyhow::Result;
use console::style;
use indicatif::{ProgressBar, FormattedDuration};

use crate::{
    model::Lockfile,
    app::{Resolvable, App, ResolvedFile, AddonType},
};

pub mod addons;
pub mod bootstrap;
pub mod scripts;
pub mod serverjar;
pub mod worlds;

#[derive(Debug)]
pub struct BuildContext<'a> {
    pub app: &'a App,

    pub output_dir: PathBuf,
    pub lockfile: Lockfile,
    pub new_lockfile: Lockfile,

    pub force: bool,
    pub skip_stages: Vec<String>,
    pub server_process: Option<Child>,
}

impl<'a> BuildContext<'a> {
    pub async fn build_all(&mut self) -> Result<String> {
        let server_name = self.app.server.name.clone();
        let banner = format!(
            "{} {}...",
            style("Building").bold(),
            style(&server_name).green().bold()
        );
        self.app.print_job(&banner)?;
        let progress_bar = self.app.multi_progress.add(ProgressBar::new_spinner())
            .with_message(banner);
        progress_bar.enable_steady_tick(Duration::from_millis(250));

        self.reload()?;

        if !self.skip_stages.is_empty() {
            //println!(" => skipping stages: {}", self.skip_stages.join(", "));
        }

        // actual stages contained here

        self.app.ci("::group::Server Jar");
        let server_jar = self.download_server_jar().await?;
        self.app.ci("::endgroup::");

        if !self.app.server.plugins.is_empty() {
            self.download_addons(AddonType::Plugin).await?;
        }

        if !self.app.server.mods.is_empty() {
            self.download_addons(AddonType::Mod).await?;
        }

        if !self.app.server.worlds.is_empty() {
            self.process_worlds().await?;
        }

        self.bootstrap_files().await?;

        if !self.app.server.launcher.disable {
            let startup = self.app.server.jar.get_startup_method(&self.app, &server_jar).await?;

            self.create_scripts(startup).await?;

            self.app.log("start.bat and start.sh created")?;
        }

        self.write_lockfile()?;

        progress_bar.disable_steady_tick();
        progress_bar.finish_and_clear();
        
        self.app.success(format!(
            "Successfully built {} in {}",
            style(&server_name).green().bold(),
            style(FormattedDuration(progress_bar.elapsed())).blue(),
        ))?;

        Ok(server_jar)
    }

    /// Load to self.lockfile and create a default one at self.new_lockfile
    pub fn reload(&mut self) -> Result<()> {
        self.lockfile = match Lockfile::get_lockfile(&self.output_dir) {
            Ok(f) => f,
            Err(_) => {
                self.app.warn("Lockfile error, using default")?;
                Lockfile {
                    path: self.output_dir.join(".mcman.lock"),
                    ..Default::default()
                }
            },
        };
        self.new_lockfile = Lockfile {
            path: self.output_dir.join(".mcman.lock"),
            ..Default::default()
        };
        Ok(())
    }

    /// Save new_lockfile
    pub fn write_lockfile(&mut self) -> Result<()> {
        self.new_lockfile.save()?;

        self.app.log("updated lockfile")?;

        Ok(())
    }

    pub async fn downloadable(
        &self,
        resolvable: &(impl Resolvable + Debug + ToString),
        folder_path: &str,
        parent_progress: Option<&ProgressBar>,
    ) -> Result<(PathBuf, ResolvedFile)> {
        let progress_bar = if let Some(parent) = parent_progress {
            self.app.multi_progress.insert_after(parent, ProgressBar::new_spinner())
        } else {
            self.app.multi_progress.add(ProgressBar::new_spinner())
        };

        let result = self.app.download(
            resolvable,
            self.output_dir.join(&folder_path),
            progress_bar
        ).await?;

        Ok((self.output_dir.join(folder_path).join(&result.filename), result))
    }
}
