use anyhow::{Result, Context};
use notify::{recommended_watcher, EventKind, Watcher, RecursiveMode};

use crate::{core::BuildContext, model::Lockfile, app::App};

use self::config::{HotReloadConfig, HotReloadAction};

pub mod config;
pub mod pattern_serde;

#[derive(Debug)]
pub struct DevSession<'a> {
    pub ctx: BuildContext<'a>,
}

impl<'a> DevSession<'a> {
    pub async fn start(mut app: App, config: HotReloadConfig) -> Result<()> {
        let mut ctx = BuildContext {
            app: &app,
            output_dir: app.server.path.join("server"),
            force: false,
            lockfile: Lockfile::default(),
            new_lockfile: Lockfile::default(),
            server_process: None,
            skip_stages: vec![],
        };

        let mut config_watcher = recommended_watcher(move |e: std::result::Result<notify::Event, notify::Error>| {
            if let Ok(e) = e {
                match e.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        for path in e.paths {
                            let Some(file) = config.files.iter().find(|f| {
                                f.path.matches_path(&path)
                            }) else {
                                return;
                            };

                            match &file.action {
                                HotReloadAction::Reload => {

                                }
                                HotReloadAction::Restart => {

                                }
                                HotReloadAction::ReloadPlugin(pl) => {

                                }
                                HotReloadAction::RunCommand(cmd) => {

                                }
                            }
                        }
                    }
                    _ => {}
                }
            } else {
                //idk
            }
        })?;

        let mut servertoml_watcher = notify::recommended_watcher(move |e: std::result::Result<notify::Event, notify::Error>| {
            let Ok(e) = e else {
                return;   
            };

            match e.kind {
                EventKind::Modify(_) => {
                    // need state or smth smh idk
                    // ctx.build_all().await?;
                }
                _ => {}
            }
        })?;

        config_watcher.watch(app.server.path.join("config").as_path(), RecursiveMode::Recursive)?;

        servertoml_watcher.watch(app.server.path.join("server.toml").as_path(), RecursiveMode::NonRecursive)?;

        Ok(())
    }
}
