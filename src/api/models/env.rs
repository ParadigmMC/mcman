use serde::{Deserialize, Serialize};

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

impl ToString for Environment {
    fn to_string(&self) -> String {
        match self {
            Environment::Both => String::from("both"),
            Environment::Server => String::from("server"),
            Environment::Client => String::from("client"),
        }
    }
}

impl Environment {
    pub fn server(&self) -> bool {
        matches!(&self, Environment::Server | Environment::Both)
    }

    pub fn client(&self) -> bool {
        matches!(&self, Environment::Client | Environment::Both)
    }
}

impl From<Env> for Environment {
    fn from(value: Env) -> Self {
        match (value.client, value.server) {
            (EnvSupport::Unsupported, EnvSupport::Optional | EnvSupport::Required) => {
                Environment::Server
            }
            (EnvSupport::Optional | EnvSupport::Required, EnvSupport::Unsupported) => {
                Environment::Client
            }
            _ => Environment::Both,
        }
    }
}
