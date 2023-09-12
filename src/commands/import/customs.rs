use anyhow::{Context, Result};
use console::style;

use crate::{
    create_http_client,
    model::{Downloadable, Server},
};

pub async fn run() -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let mut plugins = vec![];
    let mut mods = vec![];

    for dl in &server.plugins {
        plugins.push(match dl {
            Downloadable::Url { url, .. } => {
                println!(" > {}", style("Re-importing:").cyan().bold());
                println!("   {}", style(&url).dim());
                if let Ok(d) =
                    Downloadable::from_url_interactive(&http_client, &server, url, false).await
                {
                    println!(
                        "   {} {}",
                        style("-> Imported as").green(),
                        d.to_short_string()
                    );
                    d
                } else {
                    println!(" > Error occurred.");
                    dl.clone()
                }
            }
            other => other.clone(),
        });
    }

    for dl in &server.mods {
        mods.push(match dl {
            Downloadable::Url { url, .. } => {
                println!(" > {}", style("Re-importing:").cyan().bold());
                println!("   {url}");
                if let Ok(d) =
                    Downloadable::from_url_interactive(&http_client, &server, url, false).await
                {
                    println!(
                        "   {} {}",
                        style("-> Imported as").green(),
                        d.to_short_string()
                    );
                    d
                } else {
                    println!(" > Error occurred.");
                    dl.clone()
                }
            }
            other => other.clone(),
        });
    }

    server.plugins = plugins;
    server.mods = mods;

    println!(" > {}", style("Saving...").green().bold());

    server.save()?;

    println!(" > Saved!");

    server.refresh_markdown(&http_client).await?;

    Ok(())
}
