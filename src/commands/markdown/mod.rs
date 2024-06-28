use std::sync::Arc;

use anyhow::Result;

use crate::api::app::App;

pub mod render;
pub mod json;

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Render markdown files
    Render(render::Args),
    /// Save addon metadata to a file
    Json(json::Args),
}

pub async fn run(app: Arc<App>, args: Commands) -> Result<()> {
    match args {
        Commands::Render(args) => render::run(app, args).await,
        Commands::Json(args) => json::run(app, args).await,
    }
}
