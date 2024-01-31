use anyhow::Result;

use crate::app::{App, Prefix};

#[derive(clap::Args, Default)]
pub struct Args {
    pub url: Option<String>,
}

pub async fn run(mut app: App, args: Args) -> Result<()> {
    let urlstr = match args.url {
        Some(url) => url.clone(),
        None => app.prompt_string("URL or shortcode?")?,
    };

    let addon = app.dl_from_string(&urlstr).await?;
    let addon_name = addon.to_short_string();

    app.add_addon_inferred(addon)?;

    app.save_changes()?;
    app.notify(Prefix::Imported, addon_name);
    app.refresh_markdown().await?;

    Ok(())
}
