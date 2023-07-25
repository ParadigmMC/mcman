use std::{
    io::{BufRead, BufReader},
    process::Stdio,
    time::Duration,
};

use anyhow::{bail, Context, Result};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::downloadable::{sources::quilt::map_quilt_loader_version, Downloadable};

use super::{BuildContext, ReportBackState};

impl BuildContext {
    pub async fn download_server_jar(&mut self) -> Result<()> {
        let server_jar = match self.server.jar.clone() {
            Downloadable::Quilt { loader, .. } => self.jar_quilt(&loader).await,
            Downloadable::BuildTools { args } => self.jar_buildtools(&args).await,
            dl => {
                self.downloadable(&dl, None, |state, server_jar| match state {
                    ReportBackState::Skipped => {
                        println!(
                            "          Skipping server jar ({})",
                            style(server_jar.clone()).dim()
                        );
                    }
                    ReportBackState::Downloading => {
                        println!(
                            "          Downloading server jar ({})",
                            style(server_jar.clone()).dim()
                        );
                    }
                    ReportBackState::Downloaded => {}
                })
                .await
            }
        }?;

        self.server_jar_name = server_jar;

        Ok(())
    }

    pub async fn execute_child(
        &self,
        cmd: (&str, Vec<&str>),
        label: &str,
        tag: &str,
    ) -> Result<()> {
        let mut child = std::process::Command::new(cmd.0)
            .args(cmd.1)
            .current_dir(&self.output_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("Running ".to_owned() + label)?;

        let spinner = ProgressBar::new_spinner().with_style(ProgressStyle::with_template(
            "          {spinner:.dim.bold} {msg}",
        )?);

        spinner.enable_steady_tick(Duration::from_millis(200));

        let prefix = style(format!("[{tag}]")).bold();

        let mut log_file =
            File::create(self.output_dir.join(".".to_owned() + tag + ".mcman.log")).await?;

        log_file
            .write_all(format!("=== mcman {tag} / {label} output ===").as_bytes())
            .await?;

        for buf in BufReader::new(child.stdout.take().unwrap()).lines() {
            let buf = buf.unwrap();
            let buf = buf.trim();

            if !buf.is_empty() {
                log_file.write_all(buf.as_bytes()).await?;
                log_file.write_all(b"\n").await?;

                if let Some(last_line) = buf.split('\n').last() {
                    spinner.set_message(format!("{prefix} {last_line}"));
                }
            }
        }

        if !child.wait()?.success() {
            bail!("{label} exited with non-zero code");
        }

        spinner.finish_and_clear();

        Ok(())
    }

    pub async fn jar_quilt(&self, loader: &str) -> Result<String> {
        let installer_jar = self
            .downloadable(
                &self.server.jar.clone(),
                None,
                |state, filename| match state {
                    ReportBackState::Skipped => {
                        println!(
                            "          Quilt installer present ({})",
                            style(filename.clone()).dim()
                        );
                    }
                    ReportBackState::Downloading => {
                        println!(
                            "          Downloading quilt installer... ({})",
                            style(filename.clone()).dim()
                        );
                    }
                    ReportBackState::Downloaded => {}
                },
            )
            .await?;

        let mcver = self.server.mc_version.clone();

        let loader_id = map_quilt_loader_version(&self.http_client, loader)
            .await
            .context("resolving quilt loader version id (latest/latest-beta)")?;

        let server_jar = format!("quilt-server-launch-{mcver}-{loader_id}.jar");

        if !self.force && self.output_dir.join(&server_jar).exists() {
            println!(
                "          Skipping server jar ({})",
                style(server_jar.clone()).dim()
            );
        } else {
            println!(
                "          Installing quilt server... ({})",
                style(server_jar.clone()).dim()
            );

            let mut args = vec!["-jar", &installer_jar, "install", "server", &mcver];

            if loader != "latest" {
                args.push(loader);
            }

            args.push("--install-dir=.");
            args.push("--download-server");

            self.execute_child(("java", args), "Quilt server installer", "qsi")
                .await
                .context("Running quilt-server-installer")?;

            println!(
                "          Renaming... ({})",
                style("quilt-server-launch.jar => ".to_owned() + &server_jar).dim()
            );

            fs::rename(
                self.output_dir.join("quilt-server-launch.jar"),
                self.output_dir.join(&server_jar),
            )
            .await
            .context("Renaming quilt-server-launch.jar")?;
        }

        Ok(server_jar)
    }

    pub async fn jar_buildtools(&self, args: &Vec<String>) -> Result<String> {
        let installer_jar = self
            .downloadable(
                &self.server.jar.clone(),
                None,
                |state, filename| match state {
                    ReportBackState::Skipped => {
                        println!(
                            "          BuildTools present ({})",
                            style(filename.clone()).dim()
                        );
                    }
                    ReportBackState::Downloading => {
                        println!(
                            "          Downloading BuildTools... ({})",
                            style(filename.clone()).dim()
                        );
                    }
                    ReportBackState::Downloaded => {}
                },
            )
            .await?;

        let server_jar = format!("spigot-{}.jar", self.server.mc_version);

        if !self.force && self.output_dir.join(&server_jar).exists() {
            println!(
                "          Skipping server jar ({})",
                style(server_jar.clone()).dim()
            );
        } else {
            println!("          Running BuildTools...");

            let mut exec_args = vec!["-jar", &installer_jar, "--rev", &self.server.mc_version];

            for arg in args {
                exec_args.push(arg);
            }

            self.execute_child(("java", exec_args), "BuildTools", "bt")
                .await
                .context("Executing BuildTools")?;

            println!(
                "          Renaming... ({})",
                style("server.jar => ".to_owned() + &server_jar).dim()
            );

            fs::rename(
                self.output_dir.join("server.jar"),
                self.output_dir.join(&server_jar),
            )
            .await
            .context("Renaming server.jar")?;
        }

        Ok(server_jar)
    }
}
