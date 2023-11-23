use console::style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;

use crate::app::App;
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
