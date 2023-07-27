use anyhow::Result;
use indexmap::IndexMap;
use regex::Regex;

use super::{
    sources::{
        curserinth::fetch_curserinth_project,
        github::fetch_repo_description,
        jenkins::{fetch_jenkins_description, str_process_job},
        modrinth::fetch_modrinth_project,
        spigot::fetch_spigot_info,
    },
    Downloadable,
};

impl Downloadable {
    pub fn get_md_link(&self) -> String {
        match self {
            Self::Vanilla {} => "Vanilla".to_owned(),
            Self::Velocity {} => "[Velocity](https://papermc.io/software/velocity)".to_owned(),
            Self::Waterfall {} => "[Waterfall](https://papermc.io/software/waterfall)".to_owned(),
            Self::Paper {} => "[Paper](https://papermc.io/software/paper)".to_owned(),
            Self::BuildTools { .. } => {
                "[BuildTools](https://www.spigotmc.org/wiki/buildtools/)".to_owned()
            }
            Self::BungeeCord {} => {
                "[BungeeCord](https://www.spigotmc.org/wiki/bungeecord/)".to_owned()
            }
            Self::Fabric { .. } => "[Fabric](https://fabricmc.net/)".to_owned(),
            Self::Purpur { .. } => "[Purpur](https://github.com/PurpurMC/Purpur)".to_owned(),
            Self::PaperMC { project, build } => {
                format!("[PaperMC/{project}](https://github.com/PaperMC/{project}); build {build}")
            }
            Self::Quilt { .. } => "[Quilt](https://quiltmc.org/)".to_owned(),
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

    // TODO: MORE LINKS - I WANT TO ADD MORE LINKS
    // links to latest build and also build id, maybe changelogs or release page
    pub fn get_extra_jar_map(&self) -> Option<IndexMap<String, String>> {
        match self {
            Self::Jenkins {
                build, artifact, ..
            } => {
                let mut map = IndexMap::new();

                map.insert(
                    "Build".to_owned(),
                    match build.as_str() {
                        "latest" => "*Latest*".to_owned(),
                        id => format!("`#{id}`"),
                    },
                );

                if artifact != "first" {
                    map.insert("Artifact".to_owned(), format!("`{artifact}`"));
                }

                Some(map)
            }
            Self::GithubRelease { tag, asset, .. } => {
                let mut map = IndexMap::new();

                map.insert(
                    "Release".to_owned(),
                    match tag.as_str() {
                        "latest" => "*Latest*".to_owned(),
                        id => format!("`{id}`"),
                    },
                );

                if asset != "first" {
                    map.insert("Asset".to_owned(), format!("`{asset}`"));
                }

                Some(map)
            }
            Self::Fabric { loader, installer } | Self::Quilt { loader, installer } => {
                let mut map = IndexMap::new();

                map.insert(
                    "Loader".to_owned(),
                    match loader.as_str() {
                        "latest" => "*Latest*".to_owned(),
                        id => format!("`{id}`"),
                    },
                );

                if installer != "latest" {
                    map.insert("Installer".to_owned(), format!("`{installer}`"));
                }

                Some(map)
            }

            Self::PaperMC { build, .. } | Self::Purpur { build } => {
                let mut map = IndexMap::new();

                map.insert(
                    "Build".to_owned(),
                    match build.as_str() {
                        "latest" => "*Latest*".to_owned(),
                        id => format!("`#{id}`"),
                    },
                );

                Some(map)
            }

            _ => None,
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

            _ => {
                map.insert("Name".to_owned(), "Invalid Downloadable".to_owned());
            }
        };

        Ok(map)
    }

    pub fn get_type_name(&self) -> String {
        match self {
            Self::Vanilla {} => "Vanilla",
            Self::Velocity {} => "Velocity",
            Self::Waterfall {} => "Waterfall",
            Self::Paper {} => "Paper",
            Self::BungeeCord {} => "BungeeCord",
            Self::Fabric { .. } => "Fabric",
            Self::Purpur { .. } => "Purpur",
            Self::PaperMC { .. } => "PaperMC",
            Self::Quilt { .. } => "Quilt",
            Self::Url { .. } => "URL",
            Self::GithubRelease { .. } => "GithubRelease",
            Self::Jenkins { .. } => "Jenkins",
            Self::Modrinth { .. } => "Modrinth",
            Self::CurseRinth { .. } => "CurseRinth",
            Self::Spigot { .. } => "Spigot",
            Self::BuildTools { .. } => "BuildTools",
        }
        .to_owned()
    }

    pub fn fields_to_map(&self) -> IndexMap<String, String> {
        let mut map = IndexMap::new();

        map.insert("Type".to_owned(), self.get_type_name());

        match self {
            Self::Fabric { loader, installer } | Self::Quilt { loader, installer } => {
                map.insert("Loader".to_owned(), loader.clone());
                map.insert("Installer".to_owned(), installer.clone());
            }

            Self::Purpur { build } => {
                map.insert("Build".to_owned(), build.clone());
            }

            Self::PaperMC { project, build } => {
                map.insert("Project".to_owned(), project.clone());
                map.insert("Build".to_owned(), build.clone());
            }

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

            _ => {}
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

            _ => self.get_type_name(),
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
