use anyhow::{Result, Context};
use glob_match::glob_match;
use notify::{Watcher, recommended_watcher, EventKind};

use crate::{commands::build::BuildContext, create_http_client, model::Server};

use self::config::{HotReloadConfig, HotReloadAction};

pub mod config;

#[derive(Debug)]
pub struct DevSession {
    pub ctx: BuildContext,
}

impl DevSession {
    pub async fn start(config: &HotReloadConfig) -> Result<()> {
        let server = Server::load().context("Failed to load server.toml")?;
        let http_client = create_http_client()?;

        let ctx = BuildContext {
            http_client,
            output_dir: server.path.join("server"),
            server,
            ..Default::default()
        };

        let mut config_watcher = recommended_watcher(move |e: std::result::Result<notify::Event, notify::Error>| {
            if let Ok(e) = e {
                match e.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        for path in e.paths {
                            let Some(file) = config.files.iter().find(|f| {
                                glob_match(&f.path, &path.to_string_lossy())
                            }) else {
                                return;
                            };

                            match file.action {
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

        

        Ok(())
    }
}
