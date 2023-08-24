use anyhow::{Context, Result};
use console::style;

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
