use anyhow::{Context, Result};
use clap::Command;
use console::style;

use crate::{commands::version::APP_USER_AGENT, downloadable::Downloadable, model::Server};

pub fn cli() -> Command {
    Command::new("customs").about("Try to import all custom urls again")
}

pub async fn run() -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;

    let http_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;

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

    Ok(())
}
