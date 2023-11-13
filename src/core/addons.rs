use std::{collections::HashSet, time::Duration};

use anyhow::Result;
use indicatif::{ProgressIterator, ProgressBar, ProgressStyle, FormattedDuration};
use tokio::fs;

use crate::app::AddonType;

use super::BuildContext;

impl<'a> BuildContext<'a> {
    pub async fn download_addons(
        &mut self,
        addon_type: AddonType,
    ) -> Result<()> {
        let server_list = match addon_type {
            AddonType::Plugin => &self.app.server.plugins,
            AddonType::Mod => &self.app.server.mods,
        };

        self.app.print_job(&format!(
            "Processing {} {addon_type}{}...{}",
            server_list.len(),
            if server_list.len() == 1 { "" } else { "s" },
            if server_list.len() < 200 { "" } else { " may god help you" },
        ))?;

        self.app.ci(&format!("::group::{addon_type}s"));

        let mut files_list = HashSet::new();

        let pb = ProgressBar::new(server_list.len() as u64)
            .with_style(ProgressStyle::with_template("{msg} [{wide_bar:.cyan/blue}] {pos}/{len}")?)
            .with_message(format!("Processing {addon_type}s"));
        let pb = self.app.multi_progress.add(pb);
        for addon in server_list.iter().progress_with(pb.clone()) {
            let (_path, resolved) = self.downloadable(addon, &addon_type.folder(), Some(&pb)).await?;

            files_list.insert(resolved.filename.clone());

            match addon_type {
                AddonType::Plugin => &mut self.new_lockfile.plugins,
                AddonType::Mod => &mut self.new_lockfile.mods,
            }.push((addon.clone(), resolved));
        }

        let existing_files = HashSet::from_iter(match addon_type {
            AddonType::Plugin => self.lockfile.plugins.iter(),
            AddonType::Mod => self.lockfile.mods.iter(),
        }.map(|(_, res)| res.filename.clone()));

        pb.set_style(ProgressStyle::with_template("{spinner:.blue} {prefix:.yellow} {msg}")?);
        pb.set_prefix("Deleting");
        pb.enable_steady_tick(Duration::from_micros(250));
        for removed_file in existing_files.difference(&files_list) {
            pb.set_message(removed_file.clone());
            fs::remove_file(self.output_dir.join(addon_type.folder()).join(removed_file)).await?;
        }

        pb.finish_and_clear();
        if files_list.len() >= 10 {
            self.app.success(format!(
                "Processed {} {addon_type}{} in {}",
                files_list.len(),
                if files_list.len() == 1 { "" } else { "s" },
                FormattedDuration(pb.elapsed())
            ))?;
        }

        self.app.ci("::endgroup::");

        Ok(())
    }
}
