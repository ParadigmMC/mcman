use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub enum AddonMetadataSource {
    Modrinth,
    Curseforge,
    Hangar,
    Github,
    Spigot,
    #[default]
    Other,
}

impl AddonMetadataSource {
    pub fn into_str(&self) -> &'static str {
        match self {
            AddonMetadataSource::Modrinth => "modrinth",
            AddonMetadataSource::Hangar => "hangar",
            AddonMetadataSource::Spigot => "spigot",
            AddonMetadataSource::Other => "other",
            AddonMetadataSource::Github => "github",
            AddonMetadataSource::Curseforge => "curseforge",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddonMetadata {
    pub name: String,
    pub version: String,
    pub link: String,
    pub source: AddonMetadataSource,
}
