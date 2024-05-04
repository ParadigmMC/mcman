use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModpackSource {
    Local {
        modpack_type: ModpackType,
        path: String,
    },

    Remote {
        modpack_type: ModpackType,
        url: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModpackType {
    Packwiz,
    MRPack,
    Unsup,
}
