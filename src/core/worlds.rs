use std::{path::PathBuf, time::Duration};

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle, ProgressIterator};

use crate::model::World;

use super::BuildContext;

impl<'a> BuildContext<'a> {
    pub async fn process_worlds(&self) -> Result<()> {
        let progress_bar = self.app.multi_progress.add(ProgressBar::new(self.app.server.worlds.len() as u64)
            .with_style(ProgressStyle::with_template("{prefix:.bold} {msg} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?)
            .with_message("World:"));

        for (name, world) in self.app.server.worlds.iter().progress_with(progress_bar.clone()) {
            progress_bar.set_message(name.clone());

            self.process_world(
                &progress_bar,
                name,
                world
            ).await.context(format!("Processing world: {name}"))?;
        }

        Ok(())
    }

    pub async fn process_world(
        &self,
        progress_bar: &ProgressBar,
        name: &str,
        world: &World,
    ) -> Result<()> {
        if !self.world_exists_in_output(name)? {
            if self.world_source_exists(name) {
                let spinner = self.app.multi_progress.insert_after(progress_bar, ProgressBar::new_spinner()
                    .with_message("Unzipping world..."));

                spinner.enable_steady_tick(Duration::from_millis(250));

                unzip(
                    &self.app.server.path.join("worlds").join(format!("{name}.zip")),
                    &self.output_dir.join(name)
                )?;

                spinner.finish_with_message(format!("Unzipped {name}.zip successfully"));
            } else if let Some(dl) = &world.download {
                let (path, _resolved) = self.downloadable(dl, ".mcman-cache", Some(progress_bar)).await?;

                let spinner = self.app.multi_progress.insert_after(progress_bar, ProgressBar::new_spinner()
                    .with_message("Unzipping world..."));

                spinner.enable_steady_tick(Duration::from_millis(250));

                unzip(&path, &self.output_dir.join(name))?;

                spinner.finish_with_message(format!("Unzipped world successfully"));
            }
        }

        if !world.datapacks.is_empty() {
            std::fs::create_dir_all(self.output_dir.join(name).join("datapacks"))
                .context(format!("Failed to create {name}/datapacks directory"))?;

            self.process_datapacks(
                progress_bar,
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
        self.app.server.path.join("worlds").join(format!("{name}.zip")).exists()
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
        progress_bar: &ProgressBar,
        name: &str,
        world: &World,
    ) -> Result<()> {
        let pb = self.app.multi_progress.insert_after(progress_bar, ProgressBar::new(world.datapacks.len() as u64)
            .with_style(ProgressStyle::with_template("{msg} [{wide_bar:.cyan/blue}] {pos}/{len}")?)
            .with_message("Processing datapacks..."));

        for dp in world.datapacks.iter().progress_with(pb.clone()) {
            let path = format!("{name}/datapacks");
            self.downloadable(dp, &path, Some(&pb))
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
