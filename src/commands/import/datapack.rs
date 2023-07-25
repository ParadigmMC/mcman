use anyhow::{Context, Result};
use clap::{arg, ArgMatches, Command};
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{
    create_http_client,
    downloadable::Downloadable,
    model::{Server, World},
    util::SelectItem,
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

    let selected_world_name = if server.worlds.is_empty() {
        "*".to_owned()
    } else {
        let mut items: Vec<SelectItem<String>> = server
            .worlds
            .keys()
            .map(|k| SelectItem(k.clone(), k.clone()))
            .collect();

        items.push(SelectItem("*".to_owned(), "* New world entry".to_owned()));

        let idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which world to add to?")
            .items(&items)
            .default(items.len() - 1)
            .interact()?;

        items[idx].0.clone()
    };

    let world_name = if selected_world_name == "*" {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("World name?")
            .default("world".to_owned())
            .interact_text()?
    } else {
        selected_world_name
    };

    if !server.worlds.contains_key(&world_name) {
        server.worlds.insert(world_name.clone(), World::default());
    }

    server
        .worlds
        .get_mut(&world_name)
        .expect("world shouldve already been inserted")
        .datapacks
        .push(dl);

    server.save()?;

    println!(" > Datapack added to {world_name}!");

    server.refresh_markdown(&http_client).await?;

    Ok(())
}
