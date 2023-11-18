use std::{process::{Stdio, ExitStatus}, time::Duration, path::PathBuf, sync::{Mutex, Arc}};

use anyhow::{Result, Context, bail, anyhow};
use console::style;
use dialoguer::theme::ColorfulTheme;
use indicatif::ProgressBar;
use notify_debouncer_mini::{new_debouncer, Debouncer, notify::{RecommendedWatcher, RecursiveMode}, DebounceEventResult};
use pathdiff::diff_paths;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, sync::mpsc, process::Child};

use crate::core::BuildContext;

use self::config::{HotReloadConfig, HotReloadAction};

pub mod config;
pub mod pattern_serde;

#[derive(Debug)]
pub struct DevSession<'a> {
    pub builder: BuildContext<'a>,
    pub jar_name: Option<String>,
    // None to disable hot reloading
    pub hot_reload: Option<Arc<Mutex<HotReloadConfig>>>,
    // true if in test mode (exit server after server fully starts, report/upload logs on fail)
    pub test_mode: bool,
}

pub enum Command {
    Start,
    EndSession,
    Rebuild,
    SendCommand(String),
    WaitUntilExit,
    Bootstrap(PathBuf),
}

async fn try_read_line(opt: &mut Option<tokio::io::Lines<BufReader<tokio::process::ChildStdout>>>) -> Result<Option<String>> {
    match opt {
        Some(lines) => Ok(lines.next_line().await?),
        None => Ok(None),
    }
}

async fn try_wait_child(opt: &mut Option<Child>) -> Result<Option<ExitStatus>> {
    match opt {
        Some(c) => Ok(Some(c.wait().await?)),
        None => Ok(None),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TestResult {
    Success,
    Failed,
    Crashed,
}

// TODO
// [x] fix stdout nesting for some reason
// [x] commands are not being sent properly
// [x] use debouncer for notify
// [ ] reload server.toml properly
// [x] tests 

pub const LINE_CRASHED: &str = "]: Crashed! The full crash report has been saved to";

impl<'a> DevSession<'a> {
    pub async fn spawn_child(&mut self) -> Result<Child> {
        let platform = if std::env::consts::FAMILY == "windows" {
            "windows"
        } else {
            "linux"
        };

        let server_jar = self.jar_name.as_ref().unwrap().clone();
        let startup = self.builder.get_startup_method(&server_jar).await?;
        let launcher = &self.builder.app.server.launcher;
        let java = launcher.get_java();
        let args = launcher.get_arguments(&startup, platform);

        self.builder.app.dbg(format!("Running: {java} {}", args.join(" ")));

        let cwd = std::env::current_dir()?.canonicalize()?;
        // because jre is stupid
        let dir = diff_paths(&self.builder.output_dir, cwd).unwrap();

        //self.builder.app.info(&format!("Output: {}", self.builder.output_dir.display()))?;
        //self.builder.app.info(&format!("Cwd: {}", cwd.display()))?;
        //self.builder.app.info(&format!("Working directory: {}", dir.display()))?;

        Ok(
            tokio::process::Command::new(java)
            .args(args)
            .current_dir(&dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?
        )
    }

    #[allow(unused_assignments)]
    #[allow(clippy::too_many_lines)]
    async fn handle_commands(mut self, mut rx: mpsc::Receiver<Command>, mut tx: mpsc::Sender<Command>) -> Result<()> {
        let mp = self.builder.app.multi_progress.clone();

        let mut child: Option<Child> = None;
        let mut stdout_lines: Option<tokio::io::Lines<BufReader<tokio::process::ChildStdout>>> = None;

        let mut is_stopping = false;
        let mut is_session_ending = false;
        let mut test_result = TestResult::Failed;
        let mut exit_status = None;

        let mut stdin_lines = tokio::io::BufReader::new(tokio::io::stdin()).lines();

        'l: loop {
            tokio::select! {
                Some(cmd) = rx.recv() => {
                    match cmd {
                        Command::Start => {
                            self.builder.app.ci("::group::Starting server process");
                            self.builder.app.info("Starting server process...");
                            if child.is_none() {
                                let mut spawned_child = self.spawn_child().await?;
                                stdout_lines = Some(tokio::io::BufReader::new(spawned_child.stdout.take().expect("stdout None")).lines());
                                child = Some(spawned_child);
                            }
                        }
                        Command::SendCommand(command) => {
                            if let Some(ref mut child) = &mut child {
                                if let Some(ref mut stdin) = &mut child.stdin {
                                    eprintln!("checkpoint 2");
                                    let _ = stdin.write_all(command.as_bytes()).await;
                                }
                            }
                        }
                        Command::WaitUntilExit => {
                            self.builder.app.info("Waiting for process exit...");
                            is_stopping = true;
                            if let Some(ref mut child) = &mut child {
                                let should_kill = tokio::select! {
                                    () = async {
                                        // future to keep printing logs
                                        loop {
                                            if let Ok(Some(line)) = try_read_line(&mut stdout_lines).await {
                                                mp.suspend(|| {
                                                    println!(
                                                        "{}{}",
                                                        style("| ").bold(),
                                                        line.trim()
                                                    );
                                                });
                                            }
                                        }
                                    // should be unreachable since infinite loop
                                    // but still, return false, idk
                                    } => false, 
                                    status = child.wait() => {
                                        exit_status = status.ok();
                                        false
                                    },
                                    () = tokio::time::sleep(Duration::from_secs(30)) => {
                                        self.builder.app.info("Timeout reached, killing...");
                                        true
                                    },
                                    _ = tokio::signal::ctrl_c() => {
                                        self.builder.app.info("^C recieved, killing...");
                                        true
                                    },
                                };

                                if should_kill {
                                    child.kill().await?;
                                    exit_status = None;
                                }
                            }
                            is_stopping = false;
                            child = None;
                            stdout_lines = None;
                            self.builder.app.info("Server process ended");
                        }
                        Command::Rebuild => {
                            self.builder.app.info("Building...");
                            self.jar_name = Some(self.builder.build_all().await?);
                        }
                        Command::Bootstrap(path) => {
                            let rel_path = diff_paths(&path, self.builder.app.server.path.join("config"))
                                .expect("Cannot diff paths");
                            self.builder.app.info(format!("Bootstrapping: {}", rel_path.to_string_lossy().trim()));
                            if let Err(e) = self.builder.bootstrap_file(&rel_path, None).await {
                                self.builder.app.warn(format!("Error while bootstrapping:
                                - Path: {}
                                - Err: {e}", rel_path.to_string_lossy()));
                            }
                        }
                        Command::EndSession => {
                            self.builder.app.info("Ending session...");
                            self.builder.app.ci("::endgroup::");
                            break 'l;
                        }
                    }
                },
                Ok(Some(line)) = try_read_line(&mut stdout_lines) => {
                    let mut s = line.trim();

                    if self.test_mode
                        && !is_stopping
                        && test_result == TestResult::Failed {
                        if s.contains("]: Done") && s.ends_with("For help, type \"help\"") {
                            test_result = TestResult::Success;

                            self.builder.app.success("Test passed!");
    
                            tx.send(Command::SendCommand("stop\nend\n".to_owned())).await?;
                            tx.send(Command::WaitUntilExit).await?;
                            tx.send(Command::EndSession).await?;
                        } else if s.contains(LINE_CRASHED) || s == "---- end of report ----" {
                            self.builder.app.warn("Server crashed!");
                            test_result = TestResult::Crashed;

                            tx.send(Command::WaitUntilExit).await?;
                            tx.send(Command::EndSession).await?;
                        }
                    }

                    mp.suspend(|| {
                        println!(
                            "{}{s}",
                            style("| ").bold()
                        );
                    });
                },
                Ok(Some(line)) = stdin_lines.next_line() => {
                    let mut cmd = line.trim();

                    //self.builder.app.info(&format!("Sending command: {cmd}"))?;
                    if let Some(ref mut child) = &mut child {
                        if let Some(ref mut stdin) = &mut child.stdin {
                            eprintln!("checkpoint 1");
                            let _ = stdin.write_all(format!("{cmd}\n").as_bytes()).await;
                        }
                    }
                },
                Ok(Some(status)) = try_wait_child(&mut child) => {
                    exit_status = Some(status);
                    self.builder.app.ci("::endgroup::");
                    self.builder.app.info("Server process exited");

                    is_stopping = false;
                    child = None;
                    stdout_lines = None;

                    if self.test_mode {
                        tx.send(Command::EndSession).await?;
                    }
                },
                _ = tokio::signal::ctrl_c() => {
                    if is_session_ending {
                        self.builder.app.info("Force-stopping development session...");
                        break 'l;
                    } else if !is_stopping {
                        self.builder.app.info("Stopping development session...");
                        
                        tx.send(Command::SendCommand("stop\nend\n".to_owned())).await?;
                        tx.send(Command::WaitUntilExit).await?;
                        tx.send(Command::EndSession).await?;
                    }
                }
            }
        }

        // end of loop > tokio::select!

        if let Some(ref mut child) = &mut child {
            self.builder.app.info("Killing undead child process...");
            child.kill().await?;
        }

        self.builder.app.ci("::endgroup::");

        if self.test_mode {
            match test_result {
                TestResult::Success => {
                    self.builder.app.success("Test passed");
                    std::process::exit(0);
                }
                TestResult::Crashed | TestResult::Failed => {
                    mp.suspend(|| {
                        println!(
                            "{} Test failed!",
                            ColorfulTheme::default().error_prefix
                        );
    
                        if let Some(status) = &exit_status {
                            if let Some(code) = status.code() {
                                println!(
                                    "  - Process exited with code {}",
                                    if code == 0 {
                                        style(code).green()
                                    } else {
                                        style(code).red().bold()
                                    }
                                );
                            } else if !status.success() {
                                println!(
                                    "  - Process didn't exit successfully"
                                );
                            }
                        }
    
                        if let TestResult::Crashed = test_result {
                            println!(
                                "  - Server crashed"
                            );
                        }
                    });

                    if std::env::var("upload_to_mclogs") == Ok("true".to_string()) || self.builder.app.server.options.upload_to_mclogs {
                        let pb = mp.add(ProgressBar::new_spinner()
                            .with_message("Uploading to mclo.gs"));
    
                        pb.enable_steady_tick(Duration::from_millis(250));

                        let latest_log_path = self.builder.output_dir.join("logs").join("latest.log");

                        if latest_log_path.exists() {
                            let content = std::fs::read_to_string(&latest_log_path)
                                .context("Reading latest.log file")?;

                            let is_crash = if test_result == TestResult::Crashed {
                                true
                            } else {
                                content.contains(LINE_CRASHED)
                            };

                            let crash_log_path = if is_crash {
                                let folder = self.builder.output_dir.join("crash-reports");
                                if !folder.exists() {
                                    bail!("crash-reports folder doesn't exist, cant upload to mclo.gs");
                                }
    
                                // get latest crash report
                                let (report_path, _) = folder.read_dir()?
                                    .filter_map(Result::ok)
                                    .filter_map(|f| Some((f.path(), f.metadata().ok()?.modified().ok()?)))
                                    .max_by_key(|(_, t)| *t)
                                    .ok_or(anyhow!("can't find crash report"))?;
    
                                Some(report_path)
                            } else {
                                None
                            };
    
                            let log = self.builder.app.mclogs().paste_log(&content).await?;
                            drop(content);

                            let crash_log = if let Some(log_path) = crash_log_path {
                                let content = std::fs::read_to_string(&log_path)
                                    .context(format!("Reading crash log file: {}", log_path.display()))?;

                                Some(self.builder.app.mclogs().paste_log(&content).await?)
                            } else {
                                None
                            };
    
                            pb.finish_and_clear();
                            self.builder.app.log("  - Logs uploaded to mclo.gs");
                            mp.suspend(|| {
                                println!();
                                println!(" latest.log [ {} ]", log.url);
                                if let Some(log) = crash_log {
                                    println!(" crash report [ {} ]", log.url);
                                }
                                println!();
                            });
                        } else {
                            pb.finish_and_clear();
                            mp.suspend(|| println!(
                                "{} '{}' does not exist! Can't upload log.",
                                ColorfulTheme::default().error_prefix,
                                style(latest_log_path.to_string_lossy()).dim()
                            ));
                        }
                    }
    
                    std::process::exit(1);
                }
            }
        }

        Ok(())
    }

    pub fn create_hotreload_watcher(
        config: Arc<Mutex<HotReloadConfig>>,
    ) -> Result<Debouncer<RecommendedWatcher>> {
        Ok(new_debouncer(Duration::from_secs(1), move |e: DebounceEventResult| {
            if let Ok(_e) = e {
                let mut guard = config.lock().unwrap();

                match HotReloadConfig::load_from(&guard.path) {
                    Ok(updated) => {
                        eprintln!("Updated hotreload.toml :3");
                        *guard = updated;
                    }
                    Err(e) => {
                        eprintln!("hotreload.toml error: {e}");
                        eprintln!("cannot update hotreload.toml");
                    }
                }
            }
        })?)
    }

    pub fn create_config_watcher(
        config: Arc<Mutex<HotReloadConfig>>,
        tx: mpsc::Sender<Command>,
    ) -> Result<Debouncer<RecommendedWatcher>> {
        Ok(new_debouncer(Duration::from_secs(1), move |e: DebounceEventResult| {
            if let Ok(e) = e {
                for e in e {
                    let path = e.path;

                    if path.is_dir() || !path.exists() {
                        continue;
                    }

                    tx.blocking_send(Command::Bootstrap(path.clone())).unwrap();

                    let guard = config.lock().unwrap();
                    let Some(file) = guard.files.iter().find(|f| {
                        f.path.matches_path(&path)
                    }).cloned() else {
                        continue;
                    };
                    drop(guard);

                    match &file.action {
                        HotReloadAction::Reload => {
                            tx.blocking_send(Command::SendCommand("reload confirm\n".to_owned()))
                                .expect("tx send err");
                        }
                        HotReloadAction::Restart => {
                            tx.blocking_send(Command::SendCommand("stop\nend\n".to_owned()))
                                .expect("tx send err");
                            tx.blocking_send(Command::WaitUntilExit)
                                .expect("tx send err");
                            tx.blocking_send(Command::Start)
                                .expect("tx send err");
                        }
                        HotReloadAction::RunCommand(cmd) => {
                            tx.blocking_send(Command::SendCommand(format!("{cmd}\n")))
                                .expect("tx send err");
                        }
                    }
                }
            }
        })?)
    }

    pub fn create_servertoml_watcher(tx: mpsc::Sender<Command>) -> Result<Debouncer<RecommendedWatcher>> {
        Ok(new_debouncer(Duration::from_secs(1), move |e: DebounceEventResult| {
            if let Ok(e) = e {
                for _e in e {
                    tx.blocking_send(Command::SendCommand("stop\nend".to_owned()))
                        .expect("tx send err");
                    tx.blocking_send(Command::WaitUntilExit)
                        .expect("tx send err");
                    tx.blocking_send(Command::Rebuild)
                        .expect("tx send err");
                    tx.blocking_send(Command::Start)
                        .expect("tx send err");
                }
            }
        })?)
    }

    pub async fn start(mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel(32);

        if let Some(cfg_mutex) = self.hot_reload.clone() {
            let mut config_watcher = Self::create_config_watcher(cfg_mutex.clone(), tx.clone())?;
            let mut hotreload_watcher = Self::create_hotreload_watcher(cfg_mutex.clone())?;
            let mut servertoml_watcher = Self::create_servertoml_watcher(tx.clone())?;

            config_watcher.watcher().watch(self.builder.app.server.path.join("config").as_path(), RecursiveMode::Recursive)?;
            servertoml_watcher.watcher().watch(self.builder.app.server.path.join("server.toml").as_path(), RecursiveMode::NonRecursive)?;
            hotreload_watcher.watcher().watch(self.builder.app.server.path.join("hotreload.toml").as_path(), RecursiveMode::NonRecursive)?;
        }

        tx.send(Command::Rebuild).await?;
        tx.send(Command::Start).await?;

        self.handle_commands(rx, tx.clone()).await?;

        Ok(())
    }
}
