use std::time::Duration;

use anyhow::Result;
use indexmap::IndexMap;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use regex::Regex;
use tokio::io::AsyncWriteExt;

use crate::{app::App, model::{Downloadable, World}, util::{md::MarkdownTable, sanitize}};

pub struct MarkdownTemplate {
    pub id: String,
    pub table: MarkdownTable,
}

pub struct MarkdownAPI<'a>(pub &'a App);

impl<'a> MarkdownAPI<'a> {
    pub async fn update_files(&self) -> Result<()> {
        let templates = self.get_templates().await?;

        let pb = self.0.multi_progress.add(ProgressBar::new(self.0.server.markdown.files.len() as u64))
            .with_style(ProgressStyle::with_template("{prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {pos}/{len}")?)
            .with_prefix("Writing to");

        for file in self.0.server.markdown.files.iter().progress_with(pb.clone()) {
            pb.set_message(file.clone());

            let path = self.0.server.path.join(file);

            if !path.exists() {
                self.0.multi_progress.println(format!("WARN: {file} does not exist! skipping"));
                continue;
            }

            let mut content = tokio::fs::read_to_string(&path).await?;

            for MarkdownTemplate { id, table } in &templates {
                let re = Regex::new(&format!(r"(<!--start:mcman-{id}-->)([\w\W]*)(<!--end:mcman-{id}-->)"))
                    .unwrap();
                content = re.replace_all(&content, |_caps: &regex::Captures| {
                    format!("<!--start:mcman-{id}-->\n{}\n<!--end:mcman-{id}-->", table.render())
                }).to_string()
            }

            let mut f = tokio::fs::File::create(&path).await?;
            f.write_all(content.as_bytes()).await?;
        }

        Ok(())
    }

    pub async fn get_templates(&self) -> Result<Vec<MarkdownTemplate>> {
        let progress_bar = self.0.multi_progress.add(ProgressBar::new_spinner())
            .with_message("Rendering markdown...");
        progress_bar.enable_steady_tick(Duration::from_millis(250));

        let mut templates = vec![
            MarkdownTemplate {
                id: String::from("server"),
                table: self.table_server(),
            }
        ];

        if !self.0.server.mods.is_empty() || !self.0.server.plugins.is_empty() {
            templates.push(MarkdownTemplate { id: String::from("addons"), table: self.table_addons().await? });
        }

        if !self.0.server.worlds.is_empty() {
            let pb = self.0.multi_progress.insert_after(&progress_bar, ProgressBar::new(self.0.server.worlds.len() as u64))
                .with_style(ProgressStyle::with_template("  {prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {pos}/{len}")?)
                .with_prefix("World");
            
            for (world_name, world) in self.0.server.worlds.iter().progress_with(pb.clone()) {
                pb.set_message(world_name.clone());
                templates.push(MarkdownTemplate {
                    id: format!("world-{world_name}"),
                    table: self.table_world(world).await?
                });
            }
        }

        progress_bar.finish_and_clear();

        Ok(templates)
    }

    pub fn table_server(&self) -> MarkdownTable {
        let mut map = IndexMap::new();

        map.insert("Version".to_owned(), self.0.server.mc_version.clone());
        map.insert("Type".to_owned(), self.0.server.jar.get_md_link());

        map.extend(self.0.server.jar.get_metadata());

        MarkdownTable::from_map(&map)
    }

    pub async fn table_addons(
        &self
    ) -> Result<MarkdownTable> {
        let mut table = MarkdownTable::new();

        let pb = self.0.multi_progress.add(ProgressBar::new(
            (self.0.server.plugins.len() + self.0.server.mods.len()) as u64
        ))
            .with_style(ProgressStyle::with_template("  {prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {pos}/{len}")?)
            .with_prefix("Rendering addon");

        for addon in self.0.server.plugins.iter().chain(&self.0.server.mods).progress_with(pb.clone()) {
            pb.set_message(addon.to_string());
            table.add_from_map(&self.fetch_downloadable_info(addon).await?);
        }

        Ok(table)
    }

    pub async fn table_world(
        &self,
        world: &World
    ) -> Result<MarkdownTable> {
        let mut table = MarkdownTable::new();
        
        if let Some(dl) = &world.download {
            let mut map = self.fetch_downloadable_info(dl).await?;
            map.insert("Name".to_owned(), format!("**(World Download)** {}", map["Name"]));
            table.add_from_map(&map);
        }

        let pb = self.0.multi_progress.add(ProgressBar::new(
            world.datapacks.len() as u64
        ))
            .with_style(ProgressStyle::with_template("    {prefix:.blue.bold} {msg} [{wide_bar:.cyan/blue}] {pos}/{len}")?)
            .with_prefix("Rendering datapack");

        for datapack in world.datapacks.iter().progress_with(pb.clone()) {
            pb.set_message(datapack.to_string());
            table.add_from_map(&self.fetch_downloadable_info(datapack).await?);
        }

        Ok(table)
    }

    pub async fn fetch_downloadable_info(
        &self,
        dl: &Downloadable,
    ) -> Result<IndexMap<String, String>> {
        let map = match dl {
            Downloadable::Modrinth { id, version } => {
                let proj = self.0.modrinth().fetch_project(id).await?;

                IndexMap::from([
                    (
                        "Name".to_owned(),
                        format!("[{}](https://modrinth.com/mod/{})", proj.title, proj.slug),
                    ),
                    ("Description".to_owned(), sanitize(&proj.description)?),
                    ("Version".to_owned(), version.clone()),
                ])
            }

            Downloadable::CurseRinth { id, version } => {
                let proj = self.0.curserinth().fetch_project(id).await?;

                IndexMap::from([(
                    "Name".to_owned(),
                    format!("{} <sup>[CF](https://www.curseforge.com/minecraft/mc-mods/{id}) [CR](https://curserinth.kuylar.dev/mod/{id})</sup>", proj.title, id = proj.slug),
                ),
                ("Description".to_owned(), sanitize(&proj.description)?),
                ("Version".to_owned(), version.clone())])
            }

            Downloadable::Spigot { id, version } => {
                let (name, desc) = self.0.spigot().fetch_info(id).await?;

                IndexMap::from([
                    (
                        "Name".to_owned(),
                        format!("[{name}](https://www.spigotmc.org/resources/{id})"),
                    ),
                    ("Description".to_owned(), sanitize(&desc)?),
                    ("Version".to_owned(), version.clone()),
                ])
            }

            Downloadable::Hangar { id, version } => {
                let proj = mcapi::hangar::fetch_project(&self.0.http_client, id).await?;

                IndexMap::from([
                    (
                        "Name".to_owned(),
                        format!(
                            "[{}](https://hangar.papermc.io/{})",
                            proj.name,
                            proj.namespace.to_string()
                        ),
                    ),
                    ("Description".to_owned(), sanitize(&proj.description)?),
                    ("Version".to_owned(), version.clone()),
                ])
            }

            Downloadable::GithubRelease { repo, tag, asset } => {
                let desc = self.0.github().fetch_repo_description(repo).await?;

                IndexMap::from([
                    ("Name".to_owned(), dl.get_md_link()),
                    ("Description".to_owned(), sanitize(&desc)?),
                    ("Version".to_owned(), format!("{tag} / `{asset}`")),
                ])
            }

            Downloadable::Jenkins {
                url,
                job,
                build,
                artifact,
            } => {
                let desc = crate::sources::jenkins::fetch_jenkins_description(&self.0.http_client, url, job).await?;

                IndexMap::from([
                    ("Name".to_owned(), dl.get_md_link()),
                    ("Description".to_owned(), sanitize(&desc)?),
                    ("Version".to_owned(), format!("{build} / `{artifact}`")),
                ])
            }

            Downloadable::Maven { version, .. } => IndexMap::from([
                ("Name".to_owned(), dl.get_md_link()),
                ("Version".to_owned(), version.clone()),
            ]),

            Downloadable::Url {
                url,
                filename,
                desc,
            } => IndexMap::from([
                (
                    "Name".to_owned(),
                    format!(
                        "`{}`",
                        filename.as_ref().unwrap_or(&"Custom URL".to_owned())
                    ),
                ),
                (
                    "Description".to_owned(),
                    desc.as_ref()
                        .unwrap_or(&"*No description provided*".to_owned())
                        .clone(),
                ),
                ("Version".to_owned(), format!("[URL]({url})")),
            ]),
        };

        Ok(map)
    }
}
