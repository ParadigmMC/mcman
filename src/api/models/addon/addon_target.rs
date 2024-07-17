use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AddonTarget {
    Plugins,
    Mods,
    Custom(String),
}

impl Default for AddonTarget {
    fn default() -> Self {
        Self::Custom(String::new())
    }
}

impl AddonTarget {
    pub fn from_str(str: &str) -> Self {
        match str {
            "mods" => AddonTarget::Mods,
            "plugins" => AddonTarget::Plugins,
            other => AddonTarget::Custom(other.to_owned()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            AddonTarget::Mods => "mods",
            AddonTarget::Plugins => "plugins",
            AddonTarget::Custom(path) => path.as_str(),
        }
    }

    pub fn from_path(path: &str) -> Self {
        Self::from_str(
            &Path::new(path)
                .parent()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or(".".to_owned()),
        )
    }

    pub fn to_path(&self) -> PathBuf {
        PathBuf::from(self.as_str())
    }
}