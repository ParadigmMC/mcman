use std::{path::{Path, PathBuf}, sync::Arc};

use anyhow::{anyhow, Result};

use crate::api::app::App;

#[derive(clap::Args)]
pub struct BuildArgs {
    #[arg(long, default_value = "output/server")]
    pub output: PathBuf,
}

impl BuildArgs {
    pub async fn get_base_dir(&self, app: &App) -> Result<PathBuf> {
        Ok(app.server
            .read()
            .await
            .as_ref()
            .ok_or(anyhow!("No `server.toml` found"))?
            .0.parent().unwrap().to_owned()
            .join(&self.output))
    }
}

pub async fn run(app: Arc<App>, args: BuildArgs) -> Result<()> {
    let base = args.get_base_dir(&app).await?;

    log::info!("Build output: {base:?}");
    
    app.action_build(&base).await?;

    log::info!("Build complete");

    Ok(())
}
