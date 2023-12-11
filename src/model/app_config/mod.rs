use confique::Config;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Config)]
pub struct MCLogsService {
    #[config(env = "upload_to_mclogs", default = false)]
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Config)]
pub struct Services {
    #[config(nested)]
    pub mclogs: MCLogsService,
}

#[derive(Debug, Serialize, Deserialize, Config)]
pub struct AppConfig {
    #[config(default = [])]
    pub disable_cache: Vec<String>,
    #[config(nested)]
    pub services: Services,
    #[config(env = "JAVA_BIN", default = "java")]
    pub default_java: String,
}
