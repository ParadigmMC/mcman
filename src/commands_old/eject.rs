use std::fs;

use anyhow::Result;
use console::style;
use dialoguer::{theme::ColorfulTheme, Input};

use crate::app::App;

#[allow(unused_must_use)]
pub fn run(app: &App) -> Result<()> {
    if Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure you want to delete everything? This is irreversible. Type this server's name to confirm.")
        .default(String::new())
        .interact_text()? == app.server.name {
        println!(" > {}", style("Deleting server.toml...").yellow());
        let _ = fs::remove_file(app.server.path.join("server.toml"));

        println!(" > {}", style("Deleting config/...").yellow());
        let _ = fs::remove_dir_all(app.server.path.join("config"));

        println!(" > {}", style("Deleting server/...").yellow());
        let _ = fs::remove_dir_all(app.server.path.join("server"));
        println!(" > Ejected successfully.");
    } else {
        println!(" > {}", style("Cancelled").green().bold());
    }

    Ok(())
}
