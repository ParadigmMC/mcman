use serde::{Deserialize, Serialize};
use std::fmt;

use super::mrpack::{Env, EnvSupport};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    #[default]
    #[serde(alias = "*")]
    Both,
    #[serde(alias = "dedicated_server")]
    Server,
    Client,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Both => write!(f, "both"),
            Self::Server => write!(f, "server"),
            Self::Client => write!(f, "client"),
        }
    }
}

impl Environment {
    pub const fn server(self) -> bool {
        matches!(self, Self::Server | Self::Both)
    }

    pub const fn client(self) -> bool {
        matches!(self, Self::Client | Self::Both)
    }
}

impl From<Env> for Environment {
    fn from(value: Env) -> Self {
        match (value.client, value.server) {
            (EnvSupport::Unsupported, EnvSupport::Optional | EnvSupport::Required) => {
                Environment::Server
            },
            (EnvSupport::Optional | EnvSupport::Required, EnvSupport::Unsupported) => {
                Environment::Client
            },
            _ => Environment::Both,
        }
    }
}
