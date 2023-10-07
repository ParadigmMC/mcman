use std::{collections::HashSet, time::Duration};

use anyhow::{Context, Result};
use dialoguer::theme::ColorfulTheme;
use indicatif::{ProgressIterator, ProgressBar, ProgressStyle, FormattedDuration};
use tokio::fs;

use super::BuildContext;

impl<'a> BuildContext<'a> {
    pub async fn download_addons(
        &mut self,
        addon_type: &str,
    ) -> Result<()> {
        let server_list = match addon_type {
            "plugins" => &self.app.server.plugins,
            "mods" => &self.app.server.mods,
            _ => unreachable!()
        };

        let mut files_list = HashSet::new();

        let pb = ProgressBar::new(server_list.len() as u64)
            .with_style(ProgressStyle::with_template("{msg} [{wide_bar:.cyan/blue}] {pos}/{len}")?)
            .with_message(match addon_type {
                "plugins" => "Processing Plugins",
                "mods" => "Processing Mods",
                _ => unreachable!(),
            });
        let pb = self.app.multi_progress.add(pb);
        for addon in server_list.iter().progress_with(pb.clone()) {
            let (_path, resolved) = self.downloadable(addon, addon_type, Some(&pb)).await?;

            files_list.insert(resolved.filename.clone());

            match addon_type {
                "plugins" => self.new_lockfile.plugins.push((addon.clone(), resolved)),
                "mods" => self.new_lockfile.mods.push((addon.clone(), resolved)),
                _ => unreachable!(),
            }
        }

        let existing_files = HashSet::from_iter(match addon_type {
            "plugins" => self.lockfile.plugins.iter(),
            "mods" => self.lockfile.mods.iter(),
            _ => unreachable!()
        }.map(|(_, res)| res.filename.clone()));

        pb.set_style(ProgressStyle::with_template("{spinner:.blue} {prefix} {msg}")?);
        pb.set_prefix("Deleting");
        pb.enable_steady_tick(Duration::from_micros(250));
        for removed_file in existing_files.difference(&files_list) {
            pb.set_message(removed_file.clone());
            fs::remove_file(self.output_dir.join(addon_type).join(removed_file)).await?;
        }

        pb.finish_with_message(format!(
            " {} Processed {} {addon_type} in {}",
            ColorfulTheme::default().success_prefix,
            files_list.len(),
            FormattedDuration(pb.elapsed())
        ));

        /* if !removed_addons.is_empty() {
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
        } */

        Ok(())
    }
}
