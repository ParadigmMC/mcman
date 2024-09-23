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
    pub const fn supports_datapacks(self) -> bool {
        !matches!(self, Self::Proxy)
    }

    pub const fn supports_mods(self) -> bool {
        matches!(self, Self::Modded)
    }

    pub const fn supports_plugins(self) -> bool {
        matches!(self, Self::Patched | Self::Proxy)
    }

    pub const fn supports_nogui(self) -> bool {
        !matches!(self, Self::Proxy)
    }

    pub const fn supports_eula_args(self) -> bool {
        matches!(self, Self::Patched)
    }
}
