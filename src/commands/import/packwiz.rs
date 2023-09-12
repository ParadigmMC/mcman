use anyhow::{Context, Result};
use console::style;

use crate::{create_http_client, model::Server, util::packwiz::packwiz_import_from_source};

#[derive(clap::Args)]
pub struct Args {
    source: String,
}

pub async fn run(args: Args) -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let src = args.source;

    println!(" > {}", style("Importing from packwiz pack.toml...").dim());

    let (_pack, mod_count, config_count) =
        packwiz_import_from_source(&http_client, &src, &mut server).await?;

    server.save()?;

    println!(
        " > {} {} {} {} {}",
        style("Imported").green().bold(),
        style(mod_count).cyan(),
        style("mods and").green(),
        style(config_count).cyan(),
        style("config files").green(),
    );

    server.refresh_markdown(&http_client).await?;

    Ok(())
}
