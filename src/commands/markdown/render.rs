use std::sync::Arc;

use anyhow::Result;

use crate::api::app::App;

#[derive(clap::Args)]
pub struct Args {}

pub async fn run(app: Arc<App>, args: Args) -> Result<()> {
    app.render_markdown_and_save().await?;

    Ok(())
}
