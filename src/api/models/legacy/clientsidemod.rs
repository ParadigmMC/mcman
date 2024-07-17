use serde::{Deserialize, Serialize};

use super::LegacyDownloadable;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct LegacyClientSideMod {
    #[serde(flatten)]
    pub dl: LegacyDownloadable,

    pub optional: bool,
    pub desc: String,
}
