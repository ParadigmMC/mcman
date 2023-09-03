use anyhow::{Context, Result};
use console::style;
use tokio::fs;

use crate::{model::Change, util::logger::Logger};

use super::{BuildContext, ReportBackState};

impl BuildContext {
    pub async fn download_addons(
        &mut self,
        addon_type: &str,
    ) -> Result<()> {
        let change_list = match addon_type {
            "plugins" => &self.changes.plugins,
            "mods" => &self.changes.mods,
            _ => unreachable!()
        };

        let server_list = match addon_type {
            "plugins" => &self.server.plugins,
            "mods" => &self.server.mods,
            _ => unreachable!()
        };

        let addon_count = server_list.len();

        let idx_w = addon_count.to_string().len();

        let removed_addons = change_list.iter()
            .filter(|c| matches!(c, Change::Removed(_)))
            .map(|c| c.inner())
            .collect::<Vec<_>>();

        if !removed_addons.is_empty() {
            println!(
                "          {}",
                style(format!("{} {addon_type} were removed from server.toml", removed_addons.len())).dim()
            );

            let logger = Logger::List { indent: 10, len: removed_addons.len() };

            for (idx, (filename, _)) in removed_addons.iter().enumerate() {
                let path = self.output_dir.join(addon_type).join(filename);
                if path.exists() {
                    fs::remove_file(path).await?;
                    logger.item(idx, &format!(
                        "{}: {}",
                        style("Deleted   ").bold(),
                        style(filename).dim()
                    ));
                } else {
                    logger.item(idx, &format!(
                        "{}: {}",
                        style("Not Found ").bold(),
                        style(filename).dim()
                    ));
                }
            }
        }

        println!(
            "          {}",
            style(format!("{addon_count} {addon_type} present, processing...")).dim()
        );

        std::fs::create_dir_all(self.output_dir.join(addon_type))
            .context(format!("Failed to create {addon_type} directory"))?;

        for (idx, addon) in server_list
            .iter()
            .enumerate()
        {
            let filename = self.downloadable(addon, Some(addon_type), |state, file_name| match state {
                ReportBackState::Skipped => {
                    println!(
                        "          ({:idx_w$}/{addon_count}) Skipping  : {}",
                        idx + 1,
                        style(&file_name).dim()
                    );
                }
                ReportBackState::Downloaded => {
                    println!(
                        "          ({:idx_w$}/{addon_count}) {}: {}",
                        idx + 1,
                        style("Downloaded").green().bold(),
                        style(&file_name).dim()
                    );
                }
                ReportBackState::Downloading => {}
            })
            .await?;

            match addon_type {
                "plugins" => self.new_lockfile.plugins.push((filename, addon.clone())),
                "mods" => self.new_lockfile.mods.push((filename, addon.clone())),
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}
