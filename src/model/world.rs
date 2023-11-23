use serde::{Deserialize, Serialize};

use super::Downloadable;

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
#[serde(default)]
pub struct World {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub datapacks: Vec<Downloadable>,
    pub download: Option<Downloadable>,
}
