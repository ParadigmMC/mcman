use std::{process::Stdio, time::Duration, path::PathBuf};

use anyhow::{Result, Context};
use console::style;
use dialoguer::{Input, theme::{Theme, SimpleTheme}};
use notify::{recommended_watcher, EventKind, Watcher, RecursiveMode, ReadDirectoryChangesWatcher};
use pathdiff::diff_paths;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, sync::mpsc, task::JoinHandle, process::Child};

use crate::{core::BuildContext, model::Lockfile, app::App};

use self::config::{HotReloadConfig, HotReloadAction};

pub mod config;
pub mod pattern_serde;

#[derive(Debug)]
pub struct DevSession<'a> {
    pub child: Option<tokio::process::Child>,
    pub command_sender: Option<mpsc::Sender<Command>>,
    pub command_reciever: Option<mpsc::Receiver<Command>>,
    pub builder: BuildContext<'a>,
    pub jar_name: Option<String>,
}

pub enum Command {
    Start,
    Stop,
    Rebuild,
    SendCommand(String),
    WaitUntilExit,
    Bootstrap(PathBuf),
}

pub enum State {
    Starting,
    Stopping,
    Building,
    Online,
}

async fn try_read_line(opt: &mut Option<tokio::io::Lines<BufReader<tokio::process::ChildStdout>>>) -> Result<Option<String>> {
    match opt {
        Some(lines) => Ok(lines.next_line().await?),
        None => Ok(None),
    }
}

impl<'a> DevSession<'a> {
    pub async fn spawn_child(&mut self) -> Result<Child> {
        let platform = if std::env::consts::FAMILY == "windows" {
            "windows"
        } else {
            "linux"
        };

        Ok(
            tokio::process::Command::new("java")
            .args(
                self.builder.app.server
                    .launcher
                    .get_arguments(&self.builder.app.server.jar.get_startup_method(
                        &self.builder.app,
                        &self.jar_name.as_ref().unwrap().clone()
                    ).await?, platform),
            )
            .current_dir(&self.builder.output_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?
        )
    }

    async fn handle_commands(mut self, mut rx: mpsc::Receiver<Command>) -> Result<()> {
        let mp = self.builder.app.multi_progress.clone();

        let mut child: Option<Child> = None;
        //let mut child_stdout = None;

        let mut stdout_lines: Option<tokio::io::Lines<BufReader<tokio::process::ChildStdout>>> = None;

        let mut is_stopping = false;

        loop {
            tokio::select! {
                Some(cmd) = rx.recv() => {
                    match cmd {
                        Command::Start => {
                            self.builder.app.info("Starting server...")?;
                            if child.is_none() {
                                let mut spawned_child = self.spawn_child().await?;
                                stdout_lines = Some(tokio::io::BufReader::new(spawned_child.stdout.take().expect("stdout None")).lines());
                                child = Some(spawned_child);
                            }
                        }
                        Command::Stop => {
                            self.builder.app.info("Stopping server...")?;
                            if let Some(ref mut child) = &mut child {
                                child.kill().await?;
                            }
                        }
                        Command::SendCommand(command) => {
                            if let Some(ref mut child) = &mut child {
                                if let Some(ref mut stdin) = &mut child.stdin {
                                    let _ = stdin.write_all(command.as_bytes()).await;
                                }
                            }
                        }
                        Command::WaitUntilExit => {
                            self.builder.app.info("Waiting exit...")?;
                            is_stopping = true;
                            if let Some(ref mut child) = &mut child {
                                let should_kill = tokio::select! {
                                    _ = child.wait() => false,
                                    _ = tokio::time::sleep(Duration::from_secs(30)) => {
                                        self.builder.app.info("Timeout reached, killing...")?;
                                        true
                                    },
                                    _ = tokio::signal::ctrl_c() => true,
                                };

                                if should_kill {
                                    child.kill().await?;
                                }
                            }
                            is_stopping = false;
                            child = None;
                            self.builder.app.info("Server process ended")?;
                        }
                        Command::Rebuild => {
                            self.builder.app.info("Building...")?;
                            self.jar_name = Some(self.builder.build_all().await?);
                        }
                        Command::Bootstrap(path) => {
                            let rel_path = diff_paths(&path, self.builder.app.server.path.join("config"))
                                .expect("Cannot diff paths");
                            self.builder.app.info(format!("Bootstrapping: {}", rel_path.to_string_lossy()))?;
                            match self.builder.bootstrap_file(&rel_path, None).await {
                                Ok(_) => {},
                                Err(e) => self.builder.app.warn(format!("Error while bootstrapping:
                                - Path: {}
                                - Err: {e}", rel_path.to_string_lossy()))?,
                            }
                        }
                    }
                },
                Ok(Some(line)) = try_read_line(&mut stdout_lines) => {
                    let mut s = line.trim();

                    mp.suspend(|| {
                        println!(
                            "{}{s}",
                            style("| ").bold()
                        )
                    });
                },
                _ = tokio::signal::ctrl_c() => {
                    if !is_stopping {
                        break;
                    }
                }
            }
        }

        if let Some(ref mut child) = &mut child {
            self.builder.app.info("Killing undead child process...")?;
            child.kill().await?;
        }

        Ok(())
    }

    async fn create_output_task(&mut self) -> Result<()> {
        let mp = self.builder.app.multi_progress.clone();

        let child = self.child.as_mut().expect("child None");
        let mut stdout = child.stdout.take().expect("child stdout None");
        
        let reader = tokio::io::BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            mp.println(format!(
                "{}{line}",
                style("| ").bold()
            )).expect("stdout IO err");
        }

        Ok(())
    }

    async fn create_stdin_task(&mut self, tx: mpsc::Sender<Command>) -> Result<()> {
        let theme = SimpleTheme {};

        while let Ok(cmd) = Input::with_theme(&theme)
            .report(false)
            .interact_text() {
            if let Err(e) = tx.blocking_send(Command::SendCommand(cmd)) {
                eprintln!("{e}");
                break;
            }
        }

        Ok(())
    }

    pub fn create_config_watcher(
        config: HotReloadConfig,
        tx: mpsc::Sender<Command>,
    ) -> Result<ReadDirectoryChangesWatcher> {
        Ok(recommended_watcher(move |e: std::result::Result<notify::Event, notify::Error>| {
            if let Ok(e) = e {
                match e.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        for path in e.paths {
                            if path.is_dir() {
                                return;
                            }

                            tx.blocking_send(Command::Bootstrap(path.clone())).unwrap();

                            let Some(file) = config.files.iter().find(|f| {
                                f.path.matches_path(&path)
                            }) else {
                                return;
                            };

                            match &file.action {
                                HotReloadAction::Reload => {
                                    tx.blocking_send(Command::SendCommand("reload confirm".to_owned()))
                                        .expect("tx send err");
                                }
                                HotReloadAction::Restart => {
                                    tx.blocking_send(Command::SendCommand("stop\nend".to_owned()))
                                        .expect("tx send err");
                                    tx.blocking_send(Command::WaitUntilExit)
                                        .expect("tx send err");
                                    tx.blocking_send(Command::Start)
                                        .expect("tx send err");
                                }
                                HotReloadAction::RunCommand(cmd) => {
                                    tx.blocking_send(Command::SendCommand(cmd.to_owned()))
                                        .expect("tx send err");
                                }
                            }
                        }
                    }
                    _ => {}
                }
            } else {
                //idk
            }
        })?)
    }

    pub fn create_servertoml_watcher(tx: mpsc::Sender<Command>) -> Result<ReadDirectoryChangesWatcher> {
        Ok(notify::recommended_watcher(move |e: std::result::Result<notify::Event, notify::Error>| {
            let Ok(e) = e else {
                return;
            };

            match e.kind {
                EventKind::Modify(_) => {
                    tx.blocking_send(Command::SendCommand("stop\nend".to_owned()))
                        .expect("tx send err");
                    tx.blocking_send(Command::WaitUntilExit)
                        .expect("tx send err");
                    tx.blocking_send(Command::Rebuild)
                        .expect("tx send err");
                    tx.blocking_send(Command::Start)
                        .expect("tx send err");
                }
                _ => {}
            }
        })?)
    }

    pub async fn start(mut self, config: HotReloadConfig) -> Result<()> {
        let (tx, rx) = mpsc::channel(32);

        let mut config_watcher = Self::create_config_watcher(config, tx.clone())?;
        let mut servertoml_watcher = Self::create_servertoml_watcher(tx.clone())?;

        config_watcher.watch(self.builder.app.server.path.join("config").as_path(), RecursiveMode::Recursive)?;
        servertoml_watcher.watch(self.builder.app.server.path.join("server.toml").as_path(), RecursiveMode::NonRecursive)?;

        tx.send(Command::Rebuild).await?;
        tx.send(Command::Start).await?;

        self.handle_commands(rx).await?;

        Ok(())
    }
}
