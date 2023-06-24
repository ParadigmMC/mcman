use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::path::Path;

use crate::{commands::version::APP_USER_AGENT, downloadable::Downloadable, model::Server};

pub fn cli() -> Command {
    Command::new("url")
        .about("Import from an URL")
        .arg(arg!(<url>).required(true))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let mut server =
        Server::load(Path::new("server.toml")).context("Failed to load server.toml")?;

    let http_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;

    let urlstr = match matches.get_one::<String>("url") {
        Some(url) => url.clone(),
        None => Input::<String>::new().with_prompt("URL:").interact()?,
    };

    let addon = Downloadable::from_url_interactive(&http_client, &server, &urlstr).await?;

    let is_plugin = match server.jar {
        Downloadable::Fabric { .. } | Downloadable::Quilt { .. } => false,

        Downloadable::GithubRelease { .. }
        | Downloadable::Jenkins { .. }
        | Downloadable::Url { .. } => {
            Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Import as...")
                .default(0)
                .items(&["Plugin", "Mod"])
                .interact()?
                == 0
        }

        _ => true,
    };

    if is_plugin {
        server.plugins.push(addon);
    } else {
        server.mods.push(addon);
    }

    server.save(Path::new("server.toml"))?;

    println!(" > Imported!");

    Ok(())
}
