use console::style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use std::fs::File;
use std::io::Write;

use crate::app::App;
use crate::model::Server;
use anyhow::Result;

pub async fn run(mut app: App) -> Result<()> {
    if app.server.markdown.files.is_empty() {
        println!(" ! {}", style("No markdown files were found").yellow());

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add and use README.md?")
            .interact()?
        {
            app.server.markdown.files.push("README.md".to_owned());
            app.server.save()?;
        } else {
            return Ok(());
        }
    }

    app.markdown().update_files().await?;

    Ok(())
}

pub fn initialize_readme(server: &Server) -> Result<()> {
    let mut f = File::create("./README.md")?;
    let readme_content = include_str!("../../res/default_readme");
    let readme_content = readme_content
        .replace("{SERVER_NAME}", &server.name)
        .replace(
            "{ADDON_HEADER}",
            if server.jar.is_modded() {
                "Mods"
            } else {
                "Plugins"
            },
        );

    f.write_all(readme_content.as_bytes())?;

    Ok(())
}
