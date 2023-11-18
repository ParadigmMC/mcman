use indexmap::IndexMap;

use crate::{model::Downloadable, sources::jenkins::str_process_job};

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
            Self::Hangar { id, .. } => {
                format!("[{id}](https://hangar.papermc.io/{id})")
            }
            Self::Jenkins { url, job, .. } => {
                let link = url.clone() + &str_process_job(job);
                format!("[{job}]({link})")
            }
            Self::Maven { url, group, .. } => {
                format!("[{g}]({url}/{g})", g = group.replace('.', "/"))
            }
            Self::Spigot { id, .. } => {
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

    pub fn get_type_name(&self) -> String {
        match self {
            Self::Url { .. } => "URL",
            Self::GithubRelease { .. } => "GithubRel",
            Self::Jenkins { .. } => "Jenkins",
            Self::Hangar { .. } => "Hangar",
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

            Self::Modrinth { id, version }
            | Self::CurseRinth { id, version }
            | Self::Hangar { id, version }
            | Self::Spigot { id, version } => {
                map.insert("Project/URL".to_owned(), id.clone());
                map.insert("Version/Release".to_owned(), version.clone());
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

            Self::Maven {
                url,
                group,
                artifact,
                version,
                filename,
            } => {
                map.insert(
                    "Project/URL".to_owned(),
                    format!("{group}.{artifact} - ({url})"),
                );
                map.insert("Version/Release".to_owned(), version.clone());
                map.insert("Asset/File".to_owned(), filename.clone());
            }
        }

        map
    }

    pub fn to_short_string(&self) -> String {
        match self {
            Self::Modrinth { id, .. } => format!("Modrinth:{id}"),
            Self::Hangar { id, .. } => format!("Hangar:{id}"),
            Self::CurseRinth { id, .. } => format!("CurseRinth:{id}"),
            Self::Spigot { id, .. } => format!("Spigot:{id}"),
            Self::GithubRelease { repo, .. } => format!("Github:{repo}"),
            Self::Jenkins { job, .. } => format!("Jenkins:{job}"),
            Self::Url { filename, .. } => {
                if let Some(f) = filename {
                    format!("URL:{f}")
                } else {
                    "URL".to_string()
                }
            }
            Self::Maven {
                group, artifact, ..
            } => {
                format!("Maven:{group}.{artifact}")
            }
        }
    }
}

impl ToString for Downloadable {
    fn to_string(&self) -> String {
        self.to_short_string()
    }
}
