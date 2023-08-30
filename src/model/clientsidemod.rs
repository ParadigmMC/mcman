use serde::{Deserialize, Serialize};

use super::Downloadable;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct ClientSideMod {
    #[serde(flatten)]
    pub dl: Downloadable,

    pub optional: bool,
    pub desc: String,
}
