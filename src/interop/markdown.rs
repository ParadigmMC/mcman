use std::{fs::File, io::Write, time::Duration};

use anyhow::{Context, Result};
use indexmap::IndexMap;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use regex::Regex;
use tokio::io::AsyncWriteExt;

use crate::{
    app::{App, Prefix},
    model::{Downloadable, World},
    util::{md::MarkdownTable, sanitize},
};

pub struct MarkdownTemplate {
    pub id: String,
    pub table: MarkdownTable,
}

pub struct MarkdownAPI<'a>(pub &'a App);

impl<'a> MarkdownAPI<'a> {
    pub fn init_server(&self) -> Result<()> {
        let mut f = File::create(self.0.server.path.join("README.md"))?;
        let readme_content = include_str!("../../res/default_readme");
        let readme_content = readme_content
            .replace("{SERVER_NAME}", &self.0.server.name)
            .replace(
                "{ADDON_HEADER}",
                if self.0.server.jar.is_modded() {
                    "Mods"
                } else {
                    "Plugins"
                },
            );

        f.write_all(readme_content.as_bytes())?;

        Ok(())
    }

    pub fn init_network(&self) -> Result<()> {
        let mut f = File::create(self.0.network.as_ref().unwrap().path.join("README.md"))?;
        let readme_content = include_str!("../../res/default_readme_network");
        let readme_content =
            readme_content.replace("{NETWORK_NAME}", &self.0.network.as_ref().unwrap().name);

        f.write_all(readme_content.as_bytes())?;

        Ok(())
    }

    pub async fn update_files(&self) -> Result<()> {
        let templates = self.get_templates().await?;

        let pb = self
            .0
            .multi_progress
            .add(ProgressBar::new(self.0.server.markdown.files.len() as u64))
            .with_style(ProgressStyle::with_template(
                "{prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {pos}/{len}",
            )?)
            .with_prefix("Writing to");

        let mut files = self
            .0
            .server
            .markdown
            .files
            .iter()
            .map(|f| (false, self.0.server.path.join(f)))
            .collect::<Vec<_>>();
        if let Some(nw) = &self.0.network {
            files.extend(nw.markdown.files.iter().map(|f| (true, nw.path.join(f))));
        }

        for (_is_nw, path) in files.iter().progress_with(pb.clone()) {
            let filename = path.file_name().unwrap().to_string_lossy();

            pb.set_message(filename.to_string());

            if !path.exists() {
                self.0.warn(format!("{filename} does not exist! Skipping"));
                continue;
            }

            let mut content = tokio::fs::read_to_string(&path).await?;

            for MarkdownTemplate { id, table } in &templates {
                let re = Regex::new(&format!(
                    r"(<!--start:mcman-{id}-->)([\w\W]*)(<!--end:mcman-{id}-->)"
                ))
                .unwrap();
                content = re
                    .replace_all(&content, |_caps: &regex::Captures| {
                        format!(
                            "<!--start:mcman-{id}-->\n{}\n<!--end:mcman-{id}-->",
                            table.render()
                        )
                    })
                    .to_string();
            }

            let mut f = tokio::fs::File::create(&path).await?;
            f.write_all(content.as_bytes()).await?;

            self.0.notify(Prefix::Rendered, filename.to_string());
        }

        Ok(())
    }

    pub async fn get_templates(&self) -> Result<Vec<MarkdownTemplate>> {
        let progress_bar = self
            .0
            .multi_progress
            .add(ProgressBar::new_spinner())
            .with_message("Rendering markdown...");
        progress_bar.enable_steady_tick(Duration::from_millis(250));

        let mut templates = vec![MarkdownTemplate {
            id: String::from("server"),
            table: self.table_server(),
        }];

        if !self.0.server.mods.is_empty() || !self.0.server.plugins.is_empty() {
            templates.push(MarkdownTemplate {
                id: String::from("addons"),
                table: self.table_addons().await?,
            });
        }

        if !self.0.server.worlds.is_empty() {
            let pb = self
                .0
                .multi_progress
                .insert_after(
                    &progress_bar,
                    ProgressBar::new(self.0.server.worlds.len() as u64),
                )
                .with_style(ProgressStyle::with_template(
                    "  {prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {pos}/{len}",
                )?)
                .with_prefix("World");

            for (world_name, world) in self.0.server.worlds.iter().progress_with(pb.clone()) {
                pb.set_message(world_name.clone());
                templates.push(MarkdownTemplate {
                    id: format!("world-{world_name}"),
                    table: self.table_world(world).await?,
                });
            }
        }

        if self.0.network.is_some() {
            templates.push(MarkdownTemplate {
                id: "servers".to_string(),
                table: self.table_network(),
            });
        }

        progress_bar.finish_and_clear();

        Ok(templates)
    }

    pub fn table_server(&self) -> MarkdownTable {
        let mut map = IndexMap::new();

        map.insert("Version", self.0.server.mc_version.clone());
        map.insert("Type", self.0.server.jar.get_md_link());

        map.extend(self.0.server.jar.get_metadata());

        MarkdownTable::from_map(&map)
    }

    pub fn table_network(&self) -> MarkdownTable {
        let mut table = MarkdownTable::new();

        if let Some(nw) = &self.0.network {
            for (name, serv) in &nw.servers {
                let mut map = IndexMap::new();

                map.insert("Name", format!("[`{name}`](./servers/{name}/)"));
                map.insert("Port", serv.port.to_string());

                table.add_from_map(&map);
            }
        }

        table
    }

    pub async fn table_addons(&self) -> Result<MarkdownTable> {
        let mut table = MarkdownTable::new();

        let pb = self
            .0
            .multi_progress
            .add(ProgressBar::new(
                (self.0.server.plugins.len() + self.0.server.mods.len()) as u64,
            ))
            .with_style(ProgressStyle::with_template(
                "  {prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {pos}/{len}",
            )?)
            .with_prefix("Rendering addon");

        for addon in self
            .0
            .server
            .plugins
            .iter()
            .chain(&self.0.server.mods)
            .progress_with(pb.clone())
        {
            pb.set_message(addon.to_string());
            table.add_from_map(
                &self
                    .fetch_downloadable_info(addon)
                    .await
                    .context(format!("Rendering addon: {addon:#?}"))?,
            );
        }

        Ok(table)
    }

    pub async fn table_world(&self, world: &World) -> Result<MarkdownTable> {
        let mut table = MarkdownTable::with_headers(vec![
            "Name".to_owned(),
            "Description".to_owned(),
            "Version".to_owned(),
        ]);

        if let Some(dl) = &world.download {
            let mut map = self.fetch_downloadable_info(dl).await?;
            map.insert(
                "Name".to_owned(),
                format!("**(World Download)** {}", map["Name"]),
            );
            table.add_from_map(&map);
        }

        let pb = self
            .0
            .multi_progress
            .add(ProgressBar::new(world.datapacks.len() as u64))
            .with_style(ProgressStyle::with_template(
                "    {prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {pos}/{len}",
            )?)
            .with_prefix("Rendering datapack");

        for datapack in world.datapacks.iter().progress_with(pb.clone()) {
            pb.set_message(datapack.to_string());
            table.add_from_map(&self.fetch_downloadable_info(datapack).await?);
        }

        Ok(table)
    }

    #[allow(clippy::too_many_lines)]
    pub async fn fetch_downloadable_info(
        &self,
        dl: &Downloadable,
    ) -> Result<IndexMap<&'static str, String>> {
        let map = match dl {
            Downloadable::Modrinth { id, version } => {
                let proj = self.0.modrinth().fetch_project(id).await?;

                IndexMap::from([
                    (
                        "Name",
                        format!("[{}](https://modrinth.com/mod/{})", proj.title, proj.slug),
                    ),
                    ("Description", sanitize(&proj.description)?),
                    ("Version", version.clone()),
                ])
            }

            Downloadable::CurseRinth { id, version } => {
                let proj = self.0.curserinth().fetch_project(id).await?;

                IndexMap::from([(
                    "Name",
                    format!("{} <sup>[CF](https://www.curseforge.com/minecraft/mc-mods/{id}) [CR](https://curserinth.kuylar.dev/mod/{id})</sup>", proj.title, id = proj.slug),
                ),
                ("Description", sanitize(&proj.description)?),
                ("Version", version.clone())])
            }

            Downloadable::Spigot { id, version } => {
                let (name, desc) = self.0.spigot().fetch_info(id).await?;

                IndexMap::from([
                    (
                        "Name",
                        format!("[{name}](https://www.spigotmc.org/resources/{id})"),
                    ),
                    ("Description", sanitize(&desc)?),
                    ("Version", version.clone()),
                ])
            }

            Downloadable::Hangar { id, version } => {
                let proj = mcapi::hangar::fetch_project(&self.0.http_client, id).await?;

                IndexMap::from([
                    (
                        "Name",
                        format!(
                            "[{}](https://hangar.papermc.io/{})",
                            proj.name,
                            proj.namespace.to_string()
                        ),
                    ),
                    ("Description", sanitize(&proj.description)?),
                    ("Version", version.clone()),
                ])
            }

            Downloadable::GithubRelease { repo, tag, asset } => {
                let desc = self.0.github().fetch_repo_description(repo).await?;

                IndexMap::from([
                    ("Name", dl.get_md_link()),
                    ("Description", sanitize(&desc)?),
                    ("Version", format!("{tag} / `{asset}`")),
                ])
            }

            Downloadable::Jenkins {
                url,
                job,
                build,
                artifact,
            } => {
                let desc = self.0.jenkins().fetch_description(url, job).await?;

                IndexMap::from([
                    ("Name", dl.get_md_link()),
                    ("Description", sanitize(&desc)?),
                    ("Version", format!("{build} / `{artifact}`")),
                ])
            }

            Downloadable::Maven {
                version, artifact, ..
            } => IndexMap::from([
                ("Name", artifact.clone()),
                ("Description", dl.get_md_link()),
                ("Version", version.clone()),
            ]),

            Downloadable::Url {
                url,
                filename,
                desc,
            } => IndexMap::from([
                (
                    "Name",
                    format!(
                        "`{}`",
                        filename.as_ref().unwrap_or(&"Custom URL".to_owned())
                    ),
                ),
                (
                    "Description",
                    desc.as_ref()
                        .unwrap_or(&"*No description provided*".to_owned())
                        .clone(),
                ),
                ("Version", format!("[URL]({url})")),
            ]),
        };

        Ok(map)
    }
}
