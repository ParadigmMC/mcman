use anyhow::{Context, Result};
use console::style;
use dialoguer::Input;

use crate::{
    create_http_client,
    model::{Downloadable, Server},
};

#[derive(clap::Args)]
pub struct Args {
    #[arg(value_name = "SRC")]
    url: Option<String>,
}

pub async fn run(args: Args) -> Result<()> {
    let mut server = Server::load().context("Failed to load server.toml")?;
    let http_client = create_http_client()?;

    let urlstr = match args.url {
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
