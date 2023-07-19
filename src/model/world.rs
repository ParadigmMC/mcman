use serde::{Deserialize, Serialize};

use crate::downloadable::Downloadable;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct World {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub datapacks: Vec<Downloadable>,
}
