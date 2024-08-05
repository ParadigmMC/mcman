use confique::Config;

#[derive(Config)]
pub struct AppOptions {
    #[config(nested)]
    pub api_urls: ApiUrls,

    #[config(default = false, env = "MCMAN_DISABLE_CACHE")]
    pub disable_cache: bool,

    #[config(env = "GITHUB_API_TOKEN")]
    pub github_token: Option<String>,
    #[config(env = "CF_API_TOKEN")]
    pub curseforge_token: Option<String>,
    #[config(env = "MODRINTH_API_TOKEN")]
    pub modrinth_token: Option<String>,

    #[config(default = false)]
    pub use_curseforge_api: bool,
}

#[derive(Config)]
pub struct ApiUrls {
    #[config(default = "https://api.github.com", env = "API_URL_GITHUB")]
    pub github: String,
    #[config(default = "https://api.modrinth.com/v2", env = "API_URL_MODRINTH")]
    pub modrinth: String,
    #[config(default = "https://api.curse.tools/v1/cf", env = "API_URL_CURSETOOLS")]
    pub cursetools: String,
    #[config(default = "https://api.curseforge.com/v1", env = "API_URL_CURSEFORGE")]
    pub curseforge: String,
    #[config(default = "https://api.spiget.org/v2", env = "API_URL_SPIGET")]
    pub spiget: String,
    #[config(default = "https://hangar.papermc.io/api/v1", env = "API_URL_HANGAR")]
    pub hangar: String,
    #[config(default = "https://api.mclo.gs/1", env = "API_URL_MCLOGS")]
    pub mclogs: String,
    #[config(default = "https://api.papermc.io/v2", env = "API_URL_MCLOGS")]
    pub papermc: String,
    #[config(default = "https://raw.githubusercontent.com/ParadigmMC/mcman-meta/main", env = "API_URL_MCMAN_META")]
    pub mcman_meta: String,
}
