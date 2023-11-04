use anyhow::{Result, Context};

use crate::{app::App, hot_reload::{DevSession, config::HotReloadConfig}, core::BuildContext, model::Lockfile};

pub async fn run(app: App) -> Result<()> {
    let output_dir = app.server.path.join("server");

    std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    let config_path = app.server.path.join("hotreload.toml");

    let config = if config_path.exists() {
        HotReloadConfig::load_from(&config_path)?
    } else {
        app.info("Generated hotreload.toml")?;

        let cfg = HotReloadConfig {
            path: config_path,
            ..Default::default()
        };

        cfg.save()?;
        cfg
    };

    let mut builder = BuildContext {
        app: &app,
        force: false,
        skip_stages: vec![],
        output_dir,
        lockfile: Lockfile::default(),
        new_lockfile: Lockfile::default(),
        server_process: None,
    };

    let mut dev_session = DevSession {
        builder,
        child: None,
        command_reciever: None,
        command_sender: None,
        jar_name: None,
    };

    dev_session.start(config).await?;

    Ok(())
}
