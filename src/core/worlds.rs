use std::{path::PathBuf, time::Duration};

use anyhow::{Context, Result};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

use crate::{util::logger::Logger, model::World};

use super::{BuildContext, ReportBackState};

impl BuildContext {
    pub async fn process_worlds(&self) -> Result<()> {
        let world_logger = Logger::List {
            indent: 10,
            len: self.server.worlds.len(),
        };

        for (idx, (name, world)) in self.server.worlds.iter().enumerate() {
            world_logger.item(idx, &format!("{} {name}", style("World:").bold()));

            self.process_world(
                &world_logger,
                name,
                world
            ).await.context(format!("Processing world: {name}"))?;
        }

        Ok(())
    }

    pub async fn process_world(
        &self,
        world_logger: &Logger,
        name: &str,
        world: &World,
    ) -> Result<()> {
        if !self.world_exists_in_output(name)? {
            if self.world_source_exists(name) {
                let spinner = ProgressBar::new_spinner().with_style(ProgressStyle::with_template(
                    &format!(
                        "{:w$}{{spinner:.dim.bold}} {{msg}}",
                        "", w = world_logger.get_indent()
                    ),
                )?).with_message("Unzipping world...");

                spinner.enable_steady_tick(Duration::from_millis(200));

                unzip(
                    &self.server.path.join("worlds").join(format!("{name}.zip")),
                    &self.output_dir.join(name)
                )?;

                spinner.finish_with_message(format!("Unzipped {name}.zip successfully"));
            } else if let Some(dl) = &world.download {
                let filename = self.downloadable(dl, Some(".mcman-cache"), |_state, _filename| {}).await?;

                let spinner = ProgressBar::new_spinner().with_style(ProgressStyle::with_template(
                    &format!(
                        "{:w$}{{spinner:.dim.bold}} {{msg}}",
                        "", w = world_logger.get_indent()
                    ),
                )?).with_message("Unzipping world...");

                spinner.enable_steady_tick(Duration::from_millis(200));

                unzip(&self.output_dir.join(".mcman-cache").join(filename), &self.output_dir.join(name))?;

                spinner.finish_with_message(format!("Unzipped world successfully"));
            }
        }

        if !world.datapacks.is_empty() {
            std::fs::create_dir_all(self.output_dir.join(name).join("datapacks"))
                .context(format!("Failed to create {name}/datapacks directory"))?;

            self.process_datapacks(
                world_logger,
                name,
                world
            ).await.context(format!("Processing datapacks"))?;
        }

        Ok(())
    }

    pub fn world_source_exists(
        &self,
        name: &str,
    ) -> bool {
        self.server.path.join("worlds").join(format!("{name}.zip")).exists()
    }

    pub fn world_exists_in_output(
        &self,
        name: &str,
    ) -> Result<bool> {
        match self.output_dir
            .join(name)
            .metadata() {
                Ok(meta) => Ok(meta.is_dir()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
                Err(e) => Err(e.into()),
            }
    }

    pub async fn process_datapacks(
        &self,
        world_logger: &Logger,
        name: &str,
        world: &World,
    ) -> Result<()> {
        let datapacks_logger = world_logger.list(world.datapacks.len());

        for (idx, dp) in world.datapacks.iter().enumerate() {
            let path = format!("{name}/datapacks");
            self.downloadable(dp, Some(&path), |state, file_name| match state {
                ReportBackState::Skipped => {
                    datapacks_logger
                        .item(idx, &format!("Skipping    : {}", style(file_name).dim()));
                }
                ReportBackState::Downloaded => {
                    datapacks_logger.item(
                        idx,
                        &format!(
                            "{}  : {}",
                            style("Downloaded").green().bold(),
                            style(file_name).dim()
                        ),
                    );
                }
                ReportBackState::Downloading => {}
            })
            .await?;
        }

        Ok(())
    }
}

pub fn unzip(
    zipfile: &PathBuf,
    output: &PathBuf,
) -> Result<()> {
    let file = std::fs::File::open(zipfile)?;
    let mut archive = zip::ZipArchive::new(file)?;

    archive.extract(output)?;

    Ok(())
}
