use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Hash, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AddonType {
    Url {
        url: String,
    },

    #[serde(alias = "mr")]
    Modrinth {
        id: String,
        version: String,
    },
}
