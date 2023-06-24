use std::collections::HashMap;

use anyhow::Result;

use super::{
    sources::{
        jenkins::str_process_job, modrinth::fetch_modrinth_project, spigot::fetch_spigot_info,
    },
    Downloadable,
};

impl Downloadable {
    pub fn get_type_str(&self) -> String {
        match self {
            Self::Vanilla {} => "Vanilla".to_owned(),
            Self::Velocity {} => "[Velocity](https://papermc.io/software/velocity)".to_owned(),
            Self::Waterfall {} => "[Waterfall](https://papermc.io/software/waterfall)".to_owned(),
            Self::Paper {} => "[Paper](https://papermc.io/software/paper)".to_owned(),
            Self::BungeeCord {} => {
                "[BungeeCord](https://www.spigotmc.org/wiki/bungeecord/)".to_owned()
            }
            Self::Fabric { .. } => "[Fabric](https://fabricmc.net/)".to_owned(),
            Self::Purpur { .. } => "[Purpur](https://github.com/PurpurMC/Purpur)".to_owned(),
            Self::PaperMC { project, build } => {
                format!("[PaperMC/{project}](https://github.com/PaperMC/{project}); build {build}")
            }
            Self::Quilt { .. } => "[Quilt](https://quiltmc.org/)".to_owned(),
            Self::Url { url, filename } => {
                let hyperlink = format!("[URL]({url})");

                if let Some(f) = filename {
                    format!("`{f}` (Custom {hyperlink})")
                } else {
                    format!("Custom {hyperlink}]")
                }
            }
            Self::GithubRelease { repo, .. } => {
                format!("[{repo}](https://github.com/{repo})")
            }
            Self::Jenkins { url, job, .. } => {
                let link = url.clone() + &str_process_job(job);
                format!("[{job}]({link})")
            }
            _ => "?".to_owned(),
        }
    }

    // TODO: MORE LINKS - I WANT TO ADD MORE LINKS
    // links to latest build and also build id, maybe changelogs or release page
    #[allow(dead_code)] // todo: this will be added to server_info_text at readme command
    pub fn get_extra_str(self) -> Option<HashMap<String, String>> {
        match self {
            Self::Jenkins {
                build, artifact, ..
            } => {
                let mut map = HashMap::new();

                map.insert(
                    "Build".to_owned(),
                    match build.as_str() {
                        "latest" => "**Latest**".to_owned(),
                        id => format!("`#{id}`"),
                    },
                );

                if artifact != "first" {
                    map.insert("Artifact".to_owned(), format!("`{artifact}`"));
                }

                Some(map)
            }
            Self::GithubRelease { tag, asset, .. } => {
                let mut map = HashMap::new();

                map.insert(
                    "Release".to_owned(),
                    match tag.as_str() {
                        "latest" => "**Latest**".to_owned(),
                        id => format!("`{id}`"),
                    },
                );

                if asset != "first" {
                    map.insert("Asset".to_owned(), format!("`{asset}`"));
                }

                Some(map)
            }
            Self::Fabric { loader, installer } | Self::Quilt { loader, installer } => {
                let mut map = HashMap::new();

                map.insert(
                    "Loader".to_owned(),
                    match loader.as_str() {
                        "latest" => "**Latest**".to_owned(),
                        id => format!("`{id}`"),
                    },
                );

                if installer != "latest" {
                    map.insert("Installer".to_owned(), format!("`{installer}`"));
                }

                Some(map)
            }

            Self::PaperMC { build, .. } | Self::Purpur { build } => {
                let mut map = HashMap::new();

                map.insert(
                    "Build".to_owned(),
                    match build.as_str() {
                        "latest" => "**Latest**".to_owned(),
                        id => format!("`#{id}`"),
                    },
                );

                Some(map)
            }

            _ => None,
        }
    }

    pub async fn fetch_str_row(&self, client: &reqwest::Client) -> Result<Vec<String>> {
        let mut cols: Vec<String> = vec![];

        match self {
            Self::Modrinth { id, .. } => {
                let proj = fetch_modrinth_project(client, id).await?;

                cols.push(format!(
                    "[{}](https://modrinth.com/mod/{})",
                    proj.title, proj.slug
                ));

                cols.push(proj.description);
            }

            Self::Spigot { id } => {
                let (name, desc) = fetch_spigot_info(client, id).await?;

                cols.push(format!("[{name}](https://www.spigotmc.org/resources/{id})"));

                cols.push(desc);
            }

            _ => {}
        };

        Ok(cols)
    }
}
