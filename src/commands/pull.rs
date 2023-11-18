use std::{fs, path::PathBuf, time::Duration};

use anyhow::{anyhow, bail, Context, Result};
use console::style;
use dialoguer::theme::ColorfulTheme;
use glob::glob;
use indicatif::ProgressBar;
use pathdiff::diff_paths;

use crate::app::App;

#[derive(clap::Args)]
pub struct Args {
    /// Files to pull
    #[arg(required = true)]
    file: String,
}

pub fn run(app: &App, args: Args) -> Result<()> {
    let files = args.file;

    let pb = app.multi_progress.add(ProgressBar::new_spinner())
        .with_message("Pulling files...");

    pb.enable_steady_tick(Duration::from_millis(250));

    let mut count = 0;
    let mut skipped = 0;

    for entry in glob(&files)? {
        let entry = entry?;

        let diff = diff_paths(&entry, fs::canonicalize(&app.server.path)?)
            .ok_or(anyhow!("Cannot diff paths"))?;

        if !diff.starts_with("server") {
            bail!("You aren't inside server/");
        }

        let mut destination = PathBuf::new();
        let mut iter = diff.components();
        iter.next().expect("Path to have atleast 1 component");
        destination.push(&app.server.path);
        destination.push("config");
        destination.extend(iter);

        fs::create_dir_all(destination.parent().unwrap()).context("Failed to create dirs")?;

        if destination.exists() {
            if app.confirm(&format!(
                "File '{}' already exists, overwrite?",
                destination.display()
            ))? {
                app.info(format!("Skipped {}", destination.display()));
                skipped += 1;
            } else {
                continue;
            }
        }

        fs::copy(&entry, &destination)?;

        app.multi_progress.println(format!(
            " {} {} {} {}",
            ColorfulTheme::default().picked_item_prefix,
            style(&diff.to_string_lossy()).dim(),
            style("=>").bold(),
            style(
                diff_paths(
                    fs::canonicalize(&destination)?,
                    fs::canonicalize(&app.server.path)?
                )
                .unwrap_or_default()
                .to_string_lossy()
            )
            .dim()
        ))?;

        count += 1;
    }

    pb.finish_with_message(format!(
        " {} Pulled {} files to {}",
        ColorfulTheme::default().picked_item_prefix,
        count,
        style("config/").bold(),
    ));

    if skipped != 0 {
        app.warn(format!("Skipped {skipped} files"));
    }

    Ok(())
}
