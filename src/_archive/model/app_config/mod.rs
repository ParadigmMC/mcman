use confique::Config;
use serde::{Deserialize, Serialize};

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
pub struct Sources {
    #[config(nested)]
    pub github: GithubSource,
}

#[derive(Debug, Serialize, Deserialize, Config)]
pub struct GithubSource {
    #[config(env = "GITHUB_TOKEN")]
    pub api_token: Option<String>,
    #[config(env = "GITHUB_API_URL", default = "https://api.github.com")]
    pub api_url: String,
}

#[derive(Debug, Serialize, Deserialize, Config)]
pub struct AppConfig {
    #[config(default = [])]
    pub disable_cache: Vec<String>,
    #[config(nested)]
    pub services: Services,
    #[config(nested)]
    pub sources: Sources,
    #[config(env = "JAVA_BIN", default = "java")]
    pub default_java: String,
}
