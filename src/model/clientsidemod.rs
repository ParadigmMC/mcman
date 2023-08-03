use serde::{Deserialize, Serialize};

use super::Downloadable;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientSideMod {
    #[serde(flatten)]
    pub dl: Downloadable,

    pub optional: bool,
    pub desc: String,
}
