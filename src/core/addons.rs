use std::{collections::HashSet, time::Duration};

use anyhow::{bail, Result};
use indicatif::{FormattedDuration, ProgressBar, ProgressIterator, ProgressStyle};
use tokio::fs;

use crate::app::AddonType;

use super::BuildContext;

impl<'a> BuildContext<'a> {
    pub async fn download_addons(&mut self, addon_type: AddonType) -> Result<()> {
        let server_list = self.app.get_addons(addon_type);

        self.app.print_job(&format!(
            "Processing {} {addon_type}{}...{}",
            server_list.len(),
            if server_list.len() == 1 { "" } else { "s" },
            if server_list.len() < 200 {
                ""
            } else {
                " may god help you"
            },
        ));

        self.app.ci(&format!("::group::Processing {addon_type}s"));

        let mut files_list = HashSet::new();

        let pb = ProgressBar::new(server_list.len() as u64)
            .with_style(ProgressStyle::with_template(
                "{msg} [{wide_bar:.cyan/blue}] {pos}/{len}",
            )?)
            .with_message(format!("Processing {addon_type}s"));
        let pb = self.app.multi_progress.add(pb);
        for addon in server_list.iter().progress_with(pb.clone()) {
            let mut attempt = 0;
            let max_tries = std::env::var("MAX_TRIES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3);
            let (_path, resolved) = loop {
                match self
                    .downloadable(addon, &addon_type.folder(), Some(&pb))
                    .await
                {
                    Ok(d) => break d,
                    Err(e) => {
                        self.app.error(e.to_string());
                        if max_tries > attempt {
                            bail!(
                                "Max attempts reached while processing {}",
                                addon.to_short_string()
                            );
                        }
                        attempt += 1;
                    }
                }
            };

            files_list.insert(resolved.filename.clone());

            match addon_type {
                AddonType::Plugin => &mut self.new_lockfile.plugins,
                AddonType::Mod => &mut self.new_lockfile.mods,
            }
            .push((addon.clone(), resolved));
        }

        let existing_files = match addon_type {
            AddonType::Plugin => self.lockfile.plugins.iter(),
            AddonType::Mod => self.lockfile.mods.iter(),
        }
        .map(|(_, res)| res.filename.clone())
        .collect::<HashSet<_>>();

        pb.set_style(ProgressStyle::with_template(
            "{spinner:.blue} {prefix:.yellow} {msg}",
        )?);
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
            ));
        }

        self.app.ci("::endgroup::");

        Ok(())
    }
}
