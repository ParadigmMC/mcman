use std::{path::PathBuf, time::Instant};

use anyhow::{anyhow, Context, Result};
use clap::{arg, value_parser, ArgMatches, Command};
use console::style;
use tokio::fs::File;

use crate::{create_http_client, downloadable::Downloadable, model::Server, util};

pub mod serverjar;
pub mod addons;
pub mod worlds;
pub mod bootstrap;
pub mod scripts;

pub fn cli() -> Command {
    Command::new("build")
        .about("Build using server.toml configuration")
        .arg(
            arg!(-o --output [FILE] "The output directory for the server")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(--skip [stages] "Skip some stages").value_delimiter(','))
        .arg(arg!(--force "Don't skip downloading already downloaded jars"))
}

#[derive(Debug)]
pub struct BuildContext {
    pub server: Server,
    pub http_client: reqwest::Client,
    pub output_dir: PathBuf,
    pub force: bool,
    pub skip_stages: Vec<String>,
    pub start_time: Instant,
    pub stage_index: u8,
    pub server_jar_name: String,
}

impl Default for BuildContext {
    fn default() -> Self {
        Self {
            server: Server::default(),
            force: false,
            http_client: reqwest::Client::default(),
            output_dir: PathBuf::new(),
            server_jar_name: String::new(),
            skip_stages: vec![],
            stage_index: 1,
            start_time: Instant::now(),
        }
    }
}

impl BuildContext {
    pub async fn build_all(&mut self) -> Result<()> {
        self.stage_index = 1;
        self.start_time = Instant::now();

        println!(" Building {}...", style(&self.server.name).green().bold());

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
            " Successfully built {} in {}",
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
        dl: &Downloadable,
        folder_path: Option<&str>,
        report_back: F,
    ) -> Result<String> {
        let file_name = dl.get_filename(&self.server, &self.http_client).await?;

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

            util::download_with_progress(
                File::create(&file_path).await.context(format!(
                    "Failed to create output file: {}",
                    file_path.to_string_lossy()
                ))?,
                &file_name,
                dl,
                Some(&file_name),
                &self.server,
                &self.http_client,
            )
            .await?;

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

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let default_output = server.path.join("server");
    let output_dir = matches
        .get_one::<PathBuf>("output")
        .unwrap_or(&default_output)
        .clone();

    let force = matches.get_flag("force");

    let skip_stages = matches
        .get_many::<String>("skip")
        .map(|o| o.cloned().collect::<Vec<String>>())
        .unwrap_or(vec![]);

    std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    let mut ctx = BuildContext {
        server,
        http_client,
        output_dir,
        force,
        skip_stages,
        ..Default::default()
    };

    ctx.build_all().await?;

    Ok(())
}
