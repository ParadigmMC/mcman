use std::time::Duration;

use anyhow::{bail, Context, Result};
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{model::Server, util::SelectItem};

#[derive(clap::Args)]
pub struct Args {
    /// The world to unpack
    world: Option<String>,
}

pub async fn run(args: Args) -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;

    let zipfile = if let Some(s) = args.world {
        server.path.join("worlds").join(if s.ends_with(".zip") {
            s.clone()
        } else {
            format!("{s}.zip")
        })
    } else {
        let worlds = server
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
        .map(|o| o.to_string_lossy().into_owned())
        .unwrap_or("world".to_owned());
    let world_name = world_name.strip_suffix(".zip").unwrap_or(&world_name);

    let spinner = ProgressBar::new_spinner()
        .with_style(ProgressStyle::with_template(" {spinner:.dim.bold} {msg}")?)
        .with_message(format!("Unzipping world '{world_name}'..."));

    spinner.enable_steady_tick(Duration::from_millis(200));

    crate::core::worlds::unzip(&zipfile, &server.path.join("server"))?;

    spinner.finish_with_message(format!("Unzipped world '{world_name}' successfully"));

    Ok(())
}
