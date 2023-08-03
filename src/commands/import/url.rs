use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command};
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{
    create_http_client,
    model::{Downloadable, Server, SoftwareType},
};

pub fn cli() -> Command {
    Command::new("url")
        .about("Import from an URL")
        .arg(arg!(<url>).required(false))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let urlstr = match matches.get_one::<String>("url") {
        Some(url) => url.clone(),
        None => Input::<String>::new().with_prompt("URL:").interact_text()?,
    };

    let addon = Downloadable::from_url_interactive(&http_client, &server, &urlstr, false).await?;

    let is_plugin = match server.jar.get_software_type() {
        SoftwareType::Modded => false,
        SoftwareType::Normal | SoftwareType::Proxy => true,
        SoftwareType::Unknown => {
            Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Import as...")
                .default(0)
                .items(&["Plugin", "Mod"])
                .interact()?
                == 0
        }
    };

    if is_plugin {
        server.plugins.push(addon);
    } else {
        server.mods.push(addon);
    }

    server.save()?;

    server.refresh_markdown(&http_client).await?;

    println!(" > Imported!");

    Ok(())
}
