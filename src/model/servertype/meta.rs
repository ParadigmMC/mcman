use indexmap::IndexMap;

use super::{Downloadable, ServerType};

impl ServerType {
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
            Self::NeoForge { .. } => "[NeoForge](https://neoforged.net/)".to_owned(),
            Self::Forge { .. } => "[Forge](https://forums.minecraftforge.net/)".to_owned(),
            Self::Downloadable { inner } => inner.get_md_link(),
        }
    }

    pub fn get_metadata(&self) -> IndexMap<String, String> {
        let mut map = IndexMap::new();

        match self {
            Self::Fabric { loader, installer } | Self::Quilt { loader, installer } => {
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
            }

            Self::NeoForge { loader } | Self::Forge { loader } => {
                map.insert(
                    "Loader".to_owned(),
                    match loader.as_str() {
                        "latest" => "*Latest*".to_owned(),
                        id => format!("`{id}`"),
                    },
                );
            }

            Self::PaperMC { build, .. } | Self::Purpur { build } => {
                map.insert(
                    "Build".to_owned(),
                    match build.as_str() {
                        "latest" => "*Latest*".to_owned(),
                        id => format!("`#{id}`"),
                    },
                );
            }

            Self::Downloadable { inner } => match inner {
                Downloadable::Jenkins {
                    build, artifact, ..
                } => {
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
                }
                Downloadable::GithubRelease { tag, asset, .. } => {
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
                }

                _ => {}
            },

            _ => {}
        }

        map
    }
}
