use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};

use crate::App;

pub async fn init_network(app: &mut App) -> Result<()> {
    let nw = app.network.as_mut().unwrap();

    let port = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Which port should the network be on?")
        .default(25565 as u16)
        .interact_text()?;

    nw.port = port;

    nw.save()?;

    println!(
        " > {} {} {}",
        style("Network").green(),
        style(nw.name).bold(),
        style("has been initialized!").green()
    );

    println!(
        " > {} {} {}",
        style("Initialize servers in this network using").cyan(),
        style("mcman init").bold(),
        style("inside sub-folders").cyan(),
    );

    Ok(())
}
