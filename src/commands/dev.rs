use std::sync::{Arc, Mutex};

use anyhow::Result;

use crate::{app::App, hot_reload::{DevSession, config::HotReloadConfig}};

use super::run::RunArgs;

#[derive(clap::Args)]
pub struct DevArgs {
    #[command(flatten)]
    run_args: RunArgs,
}

impl<'a> DevArgs {
    pub fn create_dev_session(&self, app: &'a App) -> Result<DevSession<'a>> {
        let config_path = app.server.path.join("hotreload.toml");

        let config = if config_path.exists() {
            HotReloadConfig::load_from(&config_path)?
        } else {
            app.info("Generated hotreload.toml");

            let cfg = HotReloadConfig {
                path: config_path,
                ..Default::default()
            };

            cfg.save()?;
            cfg
        };

        let mut dev_session = self.run_args.create_dev_session(app)?;
        dev_session.hot_reload = Some(Arc::new(Mutex::new(config)));
        // no.
        dev_session.test_mode = false;

        Ok(dev_session)
    }
}

pub async fn run(app: App, args: DevArgs) -> Result<()> {
    let mut dev_session = args.create_dev_session(&app)?;

    dev_session.start().await?;

    Ok(())
}
