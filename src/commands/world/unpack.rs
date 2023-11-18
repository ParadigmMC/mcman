use std::{time::Duration, path::Path};

use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{util::SelectItem, app::App};

#[derive(clap::Args)]
pub struct Args {
    /// The world to unpack
    world: Option<String>,
}

pub fn run(app: &App, args: Args) -> Result<()> {
    let zipfile = if let Some(s) = args.world {
        app.server.path.join("worlds").join(if Path::new(&s)
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("zip")) {
            s.clone()
        } else {
            format!("{s}.zip")
        })
    } else {
        let worlds = app
            .server
            .path
            .join("worlds")
            .read_dir()?
            .collect::<Result<Vec<std::fs::DirEntry>, std::io::Error>>()?;

        if worlds.is_empty() {
            bail!("The worlds/ folder is empty, there aren't any worlds to unpack");
        } else if worlds.len() == 1 {
            worlds[0].path()
        } else {
            let items = worlds
                .iter()
                .map(|entry| {
                    SelectItem(
                        entry.path(),
                        entry.file_name().to_string_lossy().into_owned(),
                    )
                })
                .collect::<Vec<_>>();

            let idx = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which world to unpack?")
                .items(&items)
                .default(0)
                .interact()?;

            items[idx].0.clone()
        }
    };

    let world_name = zipfile
        .file_name()
        .map_or("world".to_owned(), |o| o.to_string_lossy().into_owned());
    let world_name = world_name.strip_suffix(".zip").unwrap_or(&world_name);

    let spinner = ProgressBar::new_spinner()
        .with_style(ProgressStyle::with_template(" {spinner:.dim.bold} {msg}")?)
        .with_message(format!("Unzipping world '{world_name}'..."));

    spinner.enable_steady_tick(Duration::from_millis(200));

    crate::core::worlds::unzip(&zipfile, &app.server.path.join("server"))?;

    spinner.finish_with_message(format!("Unzipped world '{world_name}' successfully"));

    Ok(())
}
