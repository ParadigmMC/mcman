use serde::{Deserialize, Serialize};

use crate::downloadable::Downloadable;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ServerLauncher {
    pub aikars_flags: bool,
    pub proxy_flags: bool,
    pub gui: bool,
    pub memory: u8,
}

impl Default for ServerLauncher {
    fn default() -> Self {
        Self {
            aikars_flags: true,
            proxy_flags: false,
            gui: false,
            memory: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Server {
    pub name: String,
    pub mc_version: String, // TODO: version type for comparing
    pub launcher: ServerLauncher,
    pub jar: Downloadable,
    pub plugins: Vec<Downloadable>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            name: String::new(),
            mc_version: "1.19.4".to_string(),
            launcher: ServerLauncher::default(),
            jar: Downloadable::Vanilla {
                version: "1.19.4".to_string(),
            },
            plugins: vec![],
        }
    }
}
