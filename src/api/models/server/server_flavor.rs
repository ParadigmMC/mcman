use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ServerFlavor {
    #[default]
    Vanilla,
    Modded,
    Patched,
    Proxy,
}

impl ServerFlavor {
    pub fn supports_datapacks(&self) -> bool {
        match self {
            ServerFlavor::Proxy => false,
            _ => true,
        }
    }

    pub fn supports_mods(&self) -> bool {
        match self {
            ServerFlavor::Modded => true,
            _ => false,
        }
    }

    pub fn supports_plugins(&self) -> bool {
        match self {
            ServerFlavor::Vanilla => false,
            ServerFlavor::Modded => false,
            ServerFlavor::Patched => true,
            ServerFlavor::Proxy => true,
        }
    }

    pub fn supports_nogui(&self) -> bool {
        match self {
            ServerFlavor::Proxy => false,
            _ => true,
        }
    }

    pub fn supports_eula_args(&self) -> bool {
        match self {
            ServerFlavor::Patched => true,
            _ => false,
        }
    }
}
