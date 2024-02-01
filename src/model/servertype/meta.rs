use indexmap::IndexMap;

use super::{Downloadable, ServerType};
use std::borrow::Cow;

macro_rules! version_id {
    ($loader:ident, |$id:ident| $id_format:literal) => {
        match loader.as_str() {
            "latest" => "*Latest*".to_owned(),
            id => format!("`{id}`"),
        }
    };

    ($loader:ident) => {
        version_id!($loader, |id| "`{id}`")
    };
}

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

    pub fn get_metadata(&self) -> IndexMap<Cow<'static, str>, String> {
        let mut map = IndexMap::new();

        match self {
            Self::Fabric { loader, installer } | Self::Quilt { loader, installer } => {
                map.insert(Cow::Borrowed("Loader"), version_id!(loader));

                if installer != "latest" {
                    map.insert(Cow::Borrowed("Installer"), format!("`{installer}`"));
                }
            }

            Self::NeoForge { loader } | Self::Forge { loader } => {
                map.insert(Cow::Borrowed("Loader"), version_id!(loader));
            }

            Self::PaperMC { build, .. } | Self::Purpur { build } => {
                map.insert(Cow::Borrowed("Build"), version_id!(build, |id| "`#{id}`"));
            }

            Self::Downloadable { inner } => match inner {
                Downloadable::Jenkins {
                    build, artifact, ..
                } => {
                    map.insert(Cow::Borrowed("Build"), version_id!(build, |id| "`#{id}`"));

                    if artifact != "first" {
                        map.insert(Cow::Borrowed("Artifact"), format!("`{artifact}`"));
                    }
                }

                Downloadable::GithubRelease { tag, asset, .. } => {
                    map.insert(Cow::Borrowed("Release"), version_id!(tag));

                    if asset != "first" {
                        map.insert(Cow::Borrowed("Asset"), format!("`{asset}`"));
                    }
                }

                _ => {}
            },

            _ => {}
        }

        map
    }
}
