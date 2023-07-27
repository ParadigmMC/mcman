use std::fs;

use anyhow::{Result, Context};
use clap::Command;
use console::style;
use dialoguer::{Input, theme::ColorfulTheme};

use crate::model::Server;

pub fn cli() -> Command {
    Command::new("eject")
        .hide(true)
        .about("Eject - remove everything related to mcman")
}

pub async fn run() -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;

    if Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to delete everything? This is irreversible. Type this server's name to confirm.")
        .default(String::new())
        .interact_text()? == server.name {
        println!(" > {}", style("Deleting server.toml...").yellow());
        _ = fs::remove_file(server.path.join("server.toml"))?;
        println!(" > {}", style("Deleting config/...").yellow());
        _ = fs::remove_dir_all(server.path.join("config"));
        println!(" > {}", style("Deleting server/...").yellow());
        _ = fs::remove_dir_all(server.path.join("server"))?;
        println!(" > Ejected successfully.");
    } else {
        println!(" > {}", style("Cancelled").green().bold());
    }

    Ok(())
}
