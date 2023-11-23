use anyhow::Result;

use crate::app::App;

#[derive(clap::Args, Default)]
pub struct Args {
    source: String,
}

pub async fn run(mut app: App, args: Args) -> Result<()> {
    app.packwiz().import_all(&args.source).await?;

    app.save_changes()?;
    app.refresh_markdown().await?;

    Ok(())
}
