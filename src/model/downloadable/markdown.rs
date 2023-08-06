use anyhow::Result;
use indexmap::IndexMap;
use regex::Regex;

use crate::{
    model::Downloadable,
    sources::{
        curserinth::fetch_curserinth_project,
        github::fetch_repo_description,
        jenkins::{fetch_jenkins_description, str_process_job},
        modrinth::fetch_modrinth_project,
        spigot::fetch_spigot_info,
    },
};

impl Downloadable {
    pub fn get_md_link(&self) -> String {
        match self {
            Self::Url { url, filename, .. } => {
                let hyperlink = format!("[URL]({url})");

                if let Some(f) = filename {
                    format!("`{f}` (Custom {hyperlink})")
                } else {
                    format!("Custom {hyperlink}")
                }
            }
            Self::GithubRelease { repo, .. } => {
                format!("[{repo}](https://github.com/{repo})")
            }
            Self::Jenkins { url, job, .. } => {
                let link = url.clone() + &str_process_job(job);
                format!("[{job}]({link})")
            }
            Self::Maven { url, group, .. } => {
                format!("[{g}]({url}/{g})", g = group.replace('.', "/"))
            }
            Self::Spigot { id } => {
                format!("[{id}](https://www.spigotmc.org/resources/{id})")
            }
            Self::Modrinth { id, .. } => {
                format!("[{id}](https://modrinth.com/mod/{id})")
            }
            Self::CurseRinth { id, .. } => {
                format!("`{id}`<sup>[CF](https://www.curseforge.com/minecraft/mc-mods/{id}) [CR](https://curserinth.kuylar.dev/mod/{id})</sup>")
            }
        }
    }

    pub async fn fetch_info_to_map(
        &self,
        client: &reqwest::Client,
    ) -> Result<IndexMap<String, String>> {
        let mut map: IndexMap<String, String> = IndexMap::new();

        match self {
            Self::Modrinth { id, version } => {
                let proj = fetch_modrinth_project(client, id).await?;

                map.insert(
                    "Name".to_owned(),
                    format!("[{}](https://modrinth.com/mod/{})", proj.title, proj.slug),
                );
                map.insert("Description".to_owned(), sanitize(&proj.description)?);
                map.insert("Version".to_owned(), version.clone());
            }

            Self::CurseRinth { id, version } => {
                let proj = fetch_curserinth_project(client, id).await?;

                map.insert(
                    "Name".to_owned(),
                    format!("{} <sup>[CF](https://www.curseforge.com/minecraft/mc-mods/{id}) [CR](https://curserinth.kuylar.dev/mod/{id})</sup>", proj.title, id = proj.slug),
                );
                map.insert("Description".to_owned(), sanitize(&proj.description)?);
                map.insert("Version".to_owned(), version.clone());
            }

            Self::Spigot { id } => {
                let (name, desc) = fetch_spigot_info(client, id).await?;

                map.insert(
                    "Name".to_owned(),
                    format!("[{name}](https://www.spigotmc.org/resources/{id})"),
                );
                map.insert("Description".to_owned(), sanitize(&desc)?);
            }

            Self::GithubRelease { repo, tag, asset } => {
                let desc = fetch_repo_description(client, repo).await?;

                map.insert("Name".to_owned(), self.get_md_link());
                map.insert("Description".to_owned(), sanitize(&desc)?);
                map.insert("Version".to_owned(), format!("{tag} / `{asset}`"));
            }

            Self::Jenkins {
                url,
                job,
                build,
                artifact,
            } => {
                let desc = fetch_jenkins_description(client, url, job).await?;

                map.insert("Name".to_owned(), self.get_md_link());
                map.insert("Description".to_owned(), sanitize(&desc)?);
                map.insert("Version".to_owned(), format!("{build} / `{artifact}`"));
            }

            Self::Maven { version, .. } => {
                map.insert("Name".to_owned(), self.get_md_link());
                map.insert("Version".to_owned(), version.to_owned());
            }

            Self::Url {
                url,
                filename,
                desc,
            } => {
                map.insert(
                    "Name".to_owned(),
                    format!(
                        "`{}`",
                        filename.as_ref().unwrap_or(&"Custom URL".to_owned())
                    ),
                );
                map.insert(
                    "Description".to_owned(),
                    desc.as_ref()
                        .unwrap_or(&"*No description provided*".to_owned())
                        .clone(),
                );
                map.insert("Version".to_owned(), format!("[URL]({url})"));
            }
        };

        Ok(map)
    }

    pub fn get_type_name(&self) -> String {
        match self {
            Self::Url { .. } => "URL",
            Self::GithubRelease { .. } => "GithubRel",
            Self::Jenkins { .. } => "Jenkins",
            Self::Modrinth { .. } => "Modrinth",
            Self::CurseRinth { .. } => "CurseRinth",
            Self::Spigot { .. } => "Spigot",
            Self::Maven { .. } => "Maven",
        }
        .to_owned()
    }

    pub fn fields_to_map(&self) -> IndexMap<String, String> {
        let mut map = IndexMap::new();

        map.insert("Type".to_owned(), self.get_type_name());

        match self {
            Self::Url { url, filename, .. } => {
                map.insert("Project/URL".to_owned(), url.clone());
                map.insert(
                    "Asset/File".to_owned(),
                    filename.as_ref().unwrap_or(&String::new()).clone(),
                );
            }

            Self::GithubRelease { repo, tag, asset } => {
                map.insert("Project/URL".to_owned(), repo.clone());
                map.insert("Version/Release".to_owned(), tag.clone());
                map.insert("Asset/File".to_owned(), asset.clone());
            }

            Self::Modrinth { id, version } | Self::CurseRinth { id, version } => {
                map.insert("Project/URL".to_owned(), id.clone());
                map.insert("Version/Release".to_owned(), version.clone());
            }

            Self::Spigot { id } => {
                map.insert("Project/URL".to_owned(), id.clone());
            }

            Self::Jenkins {
                url,
                job,
                build,
                artifact,
            } => {
                map.insert("Project/URL".to_owned(), format!("{job} - ({url})"));
                map.insert("Version/Release".to_owned(), build.clone());
                map.insert("Asset/File".to_owned(), artifact.clone());
            }

            Self::Maven { url, group, artifact, version, filename } => {
                map.insert("Project/URL".to_owned(), format!("{group}.{artifact} - ({url})"));
                map.insert("Version/Release".to_owned(), version.clone());
                map.insert("Asset/File".to_owned(), filename.clone());
            }
        }

        map
    }

    pub fn to_short_string(&self) -> String {
        match self {
            Self::Modrinth { id, .. } => format!("Modrinth/{id}"),
            Self::CurseRinth { id, .. } => format!("CurseRinth/{id}"),
            Self::Spigot { id } => format!("Spigot/{id}"),
            Self::GithubRelease { repo, .. } => format!("Github/{repo}"),
            Self::Jenkins { job, .. } => format!("Jenkins/{job}"),
            Self::Url { filename, .. } => {
                if let Some(f) = filename {
                    format!("URL/{f}")
                } else {
                    "URL".to_string()
                }
            }
            Self::Maven { group, artifact, .. } => {
                format!("Maven/{group}.{artifact}")
            }
        }
    }
}

static SANITIZE_R1: &str = "<(?:\"[^\"]*\"['\"]*|'[^']*'['\"]*|[^'\">])+>";

fn sanitize(s: &str) -> Result<String> {
    let re = Regex::new(SANITIZE_R1)?;

    Ok(re
        .replace_all(
            &s.replace('\n', " ").replace('\r', "").replace("<br>", " "),
            "",
        )
        .to_string())
}
