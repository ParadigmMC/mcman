use std::{
    io::Write,
    process::{ExitStatus, Stdio},
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use anyhow::{bail, Context, Result};
use console::style;
use std::io::{BufRead, BufReader};
use tokio::sync::oneshot;

use super::BuildContext;

impl BuildContext {
    pub fn run(&mut self, test_mode: bool) -> Result<()> {
        println!();
        println!(" {} {}...", style("> Running").bold(), self.server.name,);
        println!();

        let platform = if std::env::consts::FAMILY == "windows" {
            "windows"
        } else {
            "linux"
        };

        let child = std::process::Command::new("java")
            .args(
                self.server
                    .launcher
                    .get_arguments(&self.startup_method, platform),
            )
            .current_dir(&self.output_dir)
            .stdin(if test_mode {
                Stdio::piped()
            } else {
                Stdio::inherit()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Spawning java child process")?;

        self.server_process = Some(child);

        Ok(())
    }

    pub async fn pipe_child_process(&mut self, test_mode: bool) -> Result<ExitStatus> {
        let mut child = self
            .server_process
            .take()
            .expect("Child process to be Some()");

        let prefix = style("[MC]".to_string()).bold();

        let stdout = child.stdout.take().unwrap();
        let mut stdin = child.stdin.take();

        let test_passed = Arc::new(AtomicBool::new(false));
        let test_passed_clone = test_passed.clone();
        let (tx_stop, rx_stop) = oneshot::channel();

        // stdout
        let stdout_process = tokio::spawn(async move {
            let mut tx = Some(tx_stop);
            for buf in BufReader::new(stdout).lines() {
                let buf = buf.unwrap();
                let buf = buf.trim();

                if !buf.is_empty() {
                    // TODO: log processing lib
                    for line in buf.split('\n') {
                        println!("{prefix} {line}");

                        if test_mode
                            && !test_passed_clone.load(std::sync::atomic::Ordering::Relaxed)
                            && line.contains("]: Done")
                            && line.ends_with("For help, type \"help\"")
                            && tx.is_some()
                        {
                            println!(
                                "{} {}",
                                style("<TE>").yellow().bold(),
                                style("Server started successfully, stopping in 5s...").yellow()
                            );
                            test_passed_clone.store(true, std::sync::atomic::Ordering::Relaxed);
                            tx.take()
                                .unwrap()
                                .send(b"stop\nend\n")
                                .expect("sending stop command to other thread failed");
                        }
                    }
                }
            }
        });

        if test_mode {
            tokio::spawn(async move {
                let Ok(command) = rx_stop.await else {
                    return;
                };
                tokio::time::sleep(Duration::from_secs(5)).await;
                stdin
                    .as_mut()
                    .expect("stdin piped because test_mode")
                    .write_all(command)
                    .expect("stop/end command failed");
            });
        }

        let exit_status = child.wait()?;
        stdout_process
            .await
            .context("Awaiting stdout proxy printing thread")?;

        if !exit_status.success() {
            println!();
            println!(
                "{} {}",
                style("mcman:").cyan().bold(),
                if let Some(i) = exit_status.code() {
                    format!("java exited with code {}", style(i).red())
                } else {
                    format!(
                        "java process {} with a signal",
                        style("terminated").yellow()
                    )
                }
            );
        }

        if test_mode {
            if exit_status.success() && test_passed.load(std::sync::atomic::Ordering::Relaxed) {
                println!();
                println!(
                    "{} {}",
                    style("mcman:").cyan().bold(),
                    style("Test passed!").green()
                );
            } else {
                println!();
                println!(
                    "{} {}",
                    style("mcman:").red().bold(),
                    style("Test failed!").yellow()
                );
                bail!("Test failed");
            }
        }

        Ok(exit_status)
    }
}
