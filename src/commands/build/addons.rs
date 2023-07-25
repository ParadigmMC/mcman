use anyhow::{Context, Result};
use console::style;

use crate::commands::build::ReportBackState;

use super::BuildContext;

impl BuildContext {
    pub async fn download_addons(&self, addon_type: &str) -> Result<()> {
        let addon_count = match addon_type {
            "plugins" => self.server.plugins.len(),
            "mods" => self.server.mods.len(),
            _ => unreachable!(),
        };

        let idx_w = addon_count.to_string().len();

        println!(
            "          {}",
            style(format!("{addon_count} {addon_type} present, processing...")).dim()
        );

        std::fs::create_dir_all(self.output_dir.join(addon_type))
            .context(format!("Failed to create {addon_type} directory"))?;

        for (idx, addon) in match addon_type {
            "plugins" => &self.server.plugins,
            "mods" => &self.server.mods,
            _ => unreachable!(),
        }
        .iter()
        .enumerate()
        {
            self.downloadable(addon, Some(addon_type), |state, file_name| match state {
                ReportBackState::Skipped => {
                    println!(
                        "          ({idx:idx_w$}/{addon_count}) Skipping    : {}",
                        style(&file_name).dim()
                    );
                }
                ReportBackState::Downloaded => {
                    println!(
                        "          ({idx:idx_w$}/{addon_count}) {}  : {}",
                        style("Downloaded").green().bold(),
                        style(&file_name).dim()
                    );
                }
                ReportBackState::Downloading => {}
            })
            .await?;
        }

        Ok(())
    }
}
