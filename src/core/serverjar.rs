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

use crate::model::InstallMethod;

use super::{BuildContext, ReportBackState};

impl BuildContext {
    pub async fn download_server_jar(&mut self) -> Result<()> {
        let serverjar_name = match self
            .server
            .jar
            .get_install_method(&self.http_client, &self.server.mc_version)
            .await?
        {
            InstallMethod::Installer {
                name,
                label,
                args,
                rename_from,
                jar_name,
            } => {
                let installer_jar = self
                    .downloadable(&self.server.jar, None, |state, filename| match state {
                        ReportBackState::Skipped => {
                            println!(
                                "          {name} is present ({})",
                                style(filename.clone()).dim()
                            );
                        }
                        ReportBackState::Downloading => {
                            println!(
                                "          Downloading {name}... ({})",
                                style(filename.clone()).dim()
                            );
                        }
                        ReportBackState::Downloaded => {}
                    })
                    .await?;

                let jar_name = jar_name.replace("${mcver}", &self.server.mc_version);

                if !self.force && self.output_dir.join(&jar_name).exists() {
                    println!(
                        "          Skipping server jar ({})",
                        style(if rename_from.is_some() {
                            jar_name.clone()
                        } else {
                            "<in libraries>".to_owned()
                        })
                        .dim()
                    );
                } else {
                    println!(
                        "          Installing server jar... ({})",
                        style(if rename_from.is_some() {
                            jar_name.clone()
                        } else {
                            "<in libraries>".to_owned()
                        })
                        .dim()
                    );

                    let mut cmd_args = vec!["-jar", &installer_jar];

                    for arg in &args {
                        cmd_args.push(arg);
                    }

                    self.execute_child(("java", cmd_args.clone()), &name, &label)
                        .await
                        .context(format!("Executing command: 'java {}'", cmd_args.join(" ")))
                        .context(format!("Running installer: {name}"))?;

                    if let Some(from) = rename_from {
                        println!(
                            "          Renaming... ({})",
                            style(format!("{from} => {jar_name}")).dim()
                        );

                        fs::rename(self.output_dir.join(&from), self.output_dir.join(&jar_name))
                            .await
                            .context(format!("Renaming: {from} => {jar_name}"))?;
                    }
                }

                Ok(jar_name)
            }
            InstallMethod::SingleJar => {
                self.downloadable(&self.server.jar, None, |state, server_jar| match state {
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

        self.startup_method = self
            .server
            .jar
            .get_startup_method(&self.http_client, &serverjar_name, &self.server.mc_version)
            .await?;

        Ok(())
    }

    pub async fn execute_child(
        &self,
        cmd: (&str, Vec<&str>),
        label: &str,
        tag: &str,
    ) -> Result<()> {
        let mut child = std::process::Command::new(cmd.0)
            .args(cmd.1.iter().map(|a| self.server.format(a)))
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
            .write_all(format!("=== mcman {tag} / {label} output ===\n\n").as_bytes())
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
}
