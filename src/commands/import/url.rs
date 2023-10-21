use anyhow::Result;

use crate::app::App;

#[derive(clap::Args)]
pub struct Args {
    url: Option<String>,
}

pub async fn run(mut app: App, args: Args) -> Result<()> {
    let urlstr = match args.url {
        Some(url) => url.clone(),
        None => app.prompt_string("URL")?,
    };

    let addon = app.dl_from_string(&urlstr).await?;
    
    app.add_addon_inferred(&addon)?;

    app.save_changes()?;
    app.refresh_markdown().await?;

    app.success(format!("Added {}", addon.to_short_string()))?;

    Ok(())
}
