use std::{path::PathBuf, process::Child, time::Instant};

use anyhow::{anyhow, Context, Result};
use console::style;
use dialoguer::theme::ColorfulTheme;
use tokio::fs::{self, File};

use crate::{
    model::{Server, StartupMethod},
    util, Source,
};

pub mod addons;
pub mod bootstrap;
pub mod runner;
pub mod scripts;
pub mod serverjar;
pub mod worlds;

#[derive(Debug)]
pub struct BuildContext {
    pub server: Server,
    pub http_client: reqwest::Client,
    pub output_dir: PathBuf,
    pub force: bool,
    pub skip_stages: Vec<String>,
    pub start_time: Instant,
    pub stage_index: u8,
    pub startup_method: StartupMethod,
    pub server_process: Option<Child>,
}

impl Default for BuildContext {
    fn default() -> Self {
        Self {
            server: Server::default(),
            force: false,
            http_client: reqwest::Client::default(),
            output_dir: PathBuf::new(),
            startup_method: StartupMethod::Jar(String::from("server.jar")),
            skip_stages: vec![],
            stage_index: 1,
            start_time: Instant::now(),
            server_process: None,
        }
    }
}

impl BuildContext {
    pub async fn build_all(&mut self) -> Result<()> {
        self.stage_index = 1;
        self.start_time = Instant::now();

        println!(
            " {} {}...",
            style("Building").bold(),
            style(&self.server.name).green().bold()
        );

        if self.force {
            println!(" => {}", style("using force flag").bold());
        }

        if !self.skip_stages.is_empty() {
            println!(" => skipping stages: {}", self.skip_stages.join(", "));
        }

        // actual stages contained here

        self.run_stage("Server Jar", "serverjar").await?;

        if !self.server.plugins.is_empty() {
            self.run_stage("Plugins", "plugins").await?;
        }

        if !self.server.mods.is_empty() {
            self.run_stage("Mods", "mods").await?;
        }

        if !self.server.worlds.is_empty() {
            self.run_stage("Datapacks", "dp").await?;
        }

        self.run_stage("Configurations", "bootstrap").await?;

        if !self.server.launcher.disable {
            self.run_stage("Scripts", "scripts").await?;
        }

        println!(
            " {} Successfully built {} in {}",
            ColorfulTheme::default().success_prefix,
            style(&self.server.name).green().bold(),
            style(self.start_time.elapsed().as_millis().to_string() + "ms").blue(),
        );

        Ok(())
    }

    pub async fn run_stage(&mut self, name: &str, id: &str) -> Result<()> {
        println!(" stage {}: {}", self.stage_index, style(name).blue().bold());

        self.stage_index += 1;

        if self.skip_stages.contains(&id.to_owned()) {
            println!("      {} {id}", style("-> Skipping stage").yellow().bold());
            Ok(())
        } else {
            match id {
                "serverjar" => self
                    .download_server_jar()
                    .await
                    .context("Downloading server jar"),
                "plugins" => self
                    .download_addons("plugins")
                    .await
                    .context("Downloading plugins"),
                "mods" => self
                    .download_addons("mods")
                    .await
                    .context("Downloading mods"),
                "dp" => self
                    .process_worlds()
                    .await
                    .context("Processing worlds/datapacks"),
                "bootstrap" => self.bootstrap_files().context("Bootstrapping config files"),
                "scripts" => self
                    .create_scripts()
                    .await
                    .context("Creating launch scripts"),
                id => Err(anyhow!("Unknown build stage: {id}")),
            }
        }
    }

    pub async fn downloadable<F: Fn(ReportBackState, &str)>(
        &self,
        dl: &(impl Source + std::fmt::Debug),
        folder_path: Option<&str>,
        report_back: F,
    ) -> Result<String> {
        let file_name = dl
            .get_filename(&self.server, &self.http_client)
            .await
            .with_context(|| format!("Getting filename of Downloadable: {dl:#?}"))?;

        let file_path = if let Some(path) = folder_path {
            self.output_dir.join(path)
        } else {
            self.output_dir.clone()
        }
        .join(&file_name);

        if !self.force && file_path.exists() {
            report_back(ReportBackState::Skipped, &file_name);
        } else {
            report_back(ReportBackState::Downloading, &file_name);

            let file = File::create(&file_path).await.with_context(|| {
                format!(
                    "Failed to create output file: {}",
                    file_path.to_string_lossy()
                )
            })?;

            let result = util::download_with_progress(
                file,
                &file_name,
                dl,
                Some(&file_name),
                &self.server,
                &self.http_client,
            )
            .await
            .with_context(|| format!("Downloading Downloadable: {dl:#?}"));

            if result.is_err() {
                // try to remove file if errored
                // so we wont have any "0 bytes" files (which mcman will skip)
                _ = fs::remove_file(file_path).await;
            }

            result?;

            report_back(ReportBackState::Downloaded, &file_name);
        }

        Ok(file_name)
    }
}

pub enum ReportBackState {
    Skipped,
    Downloading,
    Downloaded,
}
