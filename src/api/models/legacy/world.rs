use serde::{Deserialize, Serialize};

use super::LegacyDownloadable;

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
#[serde(default)]
pub struct LegacyWorld {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub datapacks: Vec<LegacyDownloadable>,
    pub download: Option<LegacyDownloadable>,
}
