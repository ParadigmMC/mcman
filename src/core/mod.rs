use std::{path::PathBuf, process::Child, time::Instant, fmt::Debug};

use anyhow::{anyhow, Context, Result};
use console::style;
use dialoguer::theme::ColorfulTheme;
use indicatif::{ProgressBar, FormattedDuration};
use tokio::fs::{self, File};

use crate::{
    model::{Server, StartupMethod, Network, Lockfile, Changes},
    util::{self, logger::Logger}, app::{Resolvable, App, ResolvedFile},
};

pub mod addons;
pub mod bootstrap;
pub mod runner;
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
    pub fn from_app(app: &'a App) -> Self {
        Self {
            app,
            force: false,
            output_dir: PathBuf::new(),
            lockfile: Lockfile::default(),
            new_lockfile: Lockfile::default(),
            skip_stages: vec![],
            server_process: None,
        }
    }
}

impl<'a> BuildContext<'a> {
    pub async fn build_all(&'a mut self) -> Result<()> {
        let server_name = self.app.server.name.clone();
        let progress_bar = self.app.multi_progress.add(ProgressBar::new_spinner())
            .with_message(format!(
                "{} {}...",
                style("Building").bold(),
                style(&server_name).green().bold()
            ));

        self.reload()?;

        if !self.skip_stages.is_empty() {
            //println!(" => skipping stages: {}", self.skip_stages.join(", "));
        }

        // actual stages contained here

        let server_jar = self.download_server_jar().await?;

        if !self.app.server.plugins.is_empty() {
            self.download_addons("plugins").await?;
        }

        if !self.app.server.mods.is_empty() {
            self.download_addons("mods").await?;
        }

        // TODO worlds/datapacks

        self.bootstrap_files().await?;

        if !self.app.server.launcher.disable {
            let startup = self.app.server.jar.get_startup_method(&self.app, &server_jar).await?;

            self.create_scripts(startup).await?;

            progress_bar.println(format!(
                "   start.bat and start.sh created"
            ));
        }

        self.write_lockfile()?;

        progress_bar.finish_with_message(format!(
            " {} Successfully built {} in {}",
            ColorfulTheme::default().success_prefix,
            style(&server_name).green().bold(),
            style(FormattedDuration(progress_bar.elapsed())).blue(),
        ));

        Ok(())
    }

    pub fn reload(&mut self) -> Result<()> {
        self.lockfile = Lockfile::get_lockfile(&self.output_dir)?;
        self.new_lockfile = Lockfile {
            path: self.output_dir.join(".mcman.lock"),
            ..Default::default()
        };
        Ok(())
    }

    pub fn write_lockfile(&mut self) -> Result<()> {
        self.new_lockfile.save()?;

        println!(
            "          {}",
            style("updated lockfile").dim()
        );

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
