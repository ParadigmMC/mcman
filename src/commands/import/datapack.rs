use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command};
use console::style;
use dialoguer::Input;

use crate::{
    create_http_client,
    model::{Downloadable, Server},
};

pub fn cli() -> Command {
    Command::new("datapack")
        .about("Import datapack from url")
        .visible_alias("dp")
        .arg(arg!(<url>).required(false))
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let urlstr = match matches.get_one::<String>("url") {
        Some(url) => url.clone(),
        None => Input::<String>::new().with_prompt("URL:").interact_text()?,
    };

    let dl = Downloadable::from_url_interactive(&http_client, &server, &urlstr, true).await?;

    let world_name = server.add_datapack(dl.clone())?;

    server.save()?;

    println!(
        " > {} {} {} {world_name}{}",
        style("Datapack from").green(),
        dl.to_short_string(),
        style("added to").green(),
        style("!").green()
    );

    server.refresh_markdown(&http_client).await?;

    Ok(())
}
