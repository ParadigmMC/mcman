use std::{path::{Path, PathBuf}, sync::Arc};

use anyhow::Result;

use crate::api::app::App;

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, default_value = "output/server")]
    pub output: PathBuf,
}

pub async fn run(app: Arc<App>, args: Args) -> Result<()> {
    let Some(server_path) = app.server.read().await.as_ref().map(|(path, _)| path.parent().unwrap().to_owned()) else {
        log::error!("No `server.toml` found to build server");
        return Ok(())
    };

    let base = server_path.join(&args.output);

    log::info!("Build output: {base:?}");
    
    app.action_build(&base).await?;

    Ok(())
}
