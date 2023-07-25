use anyhow::{Context, Result};
use console::style;

use super::{BuildContext, ReportBackState};

impl BuildContext {
    pub async fn process_worlds(&self) -> Result<()> {
        let world_count = self.server.worlds.len();
        let wc_len = world_count.to_string().len();

        for (idx, (name, world)) in self.server.worlds.iter().enumerate() {
            println!(
                "          ({idx:wc_len$}/{world_count}) {} {name}",
                style("World:").bold()
            );

            std::fs::create_dir_all(self.output_dir.join(name).join("datapacks"))
                .context(format!("Failed to create {name}/datapacks directory"))?;

            let datapack_count = world.datapacks.len();
            let dp_len = datapack_count.to_string().len();
            let pad_len = wc_len * 2 + 4;

            for (idx, dp) in world.datapacks.iter().enumerate() {
                let path = format!("{name}/datapacks");
                self.downloadable(
                    dp,
                    Some(&path),
                    |state, file_name| {
                    match state {
                        ReportBackState::Skipped => {
                            println!(
                                "          {:pad_len$}({idx:dp_len$}/{datapack_count}) Skipping    : {}",
                                "",
                                style(file_name).dim()
                            );
                        }
                        ReportBackState::Downloaded => {
                            println!(
                                "          {:pad_len$}({idx:dp_len$}/{datapack_count}) {}  : {}",
                                "",
                                style("Downloaded").green().bold(),
                                style(file_name).dim()
                            );
                        }
                        ReportBackState::Downloading => {}
                    }
                }).await?;
            }
        }

        Ok(())
    }
}
