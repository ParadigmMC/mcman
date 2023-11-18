mod hashing;
mod downloading;
mod progress;
mod from_string;
mod caching;
mod resolvable;
mod actions;
mod feedback;

use anyhow::{Result, Context};
use indicatif::MultiProgress;
pub use resolvable::*;
pub use caching::*;
pub use feedback::*;

use crate::sources;
use crate::model::{Network, Server};

pub const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " - ",
    env!("CARGO_PKG_REPOSITORY"),
);

#[derive(Debug, Clone, Copy)]
pub enum AddonType {
    Plugin,
    Mod,
}

impl AddonType {
    pub fn folder(&self) -> String {
        match self {
            Self::Mod => String::from("mods"),
            Self::Plugin => String::from("plugins"),
        }
    }
}

impl std::fmt::Display for AddonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Plugin => "plugin",
            Self::Mod => "mod",
        })
    }
}

pub struct BaseApp {
    pub http_client: reqwest::Client,
}

impl BaseApp {
    pub fn new() -> Result<Self> {
        let b = reqwest::Client::builder().user_agent(APP_USER_AGENT);

        Ok(Self {
            http_client: b.build().context("Failed to create HTTP client")?
        })
    }

    pub fn upgrade(self) -> Result<App> {
        Ok(App {
            http_client: self.http_client,
            server: Server::load().context("Failed to load server.toml")?,
            network: Network::load()?,
            multi_progress: MultiProgress::new(),
        })
    }

    pub fn upgrade_with_default_server(self) -> Result<App> {
        Ok(App {
            http_client: self.http_client,
            server: Server::default(),
            network: Network::load()?,
            multi_progress: MultiProgress::new(),
        })
    }
}

#[derive(Debug)]
pub struct App {
    pub http_client: reqwest::Client,
    pub server: Server,
    pub network: Option<Network>,

    pub multi_progress: MultiProgress,
}

impl App {
    pub fn mc_version(&self) -> String {
        self.server.mc_version.clone()
    }

    pub fn var<I: AsRef<str>>(&self, var: I) -> Option<String> {
        let k = var.as_ref();
        match k {
            "SERVER_NAME" => Some(self.server.name.clone()),
            "SERVER_VERSION" | "mcver" | "mcversion" => Some(self.server.mc_version.clone()),

            "SERVER_PORT" => {
                std::env::var(format!("PORT_{}", self.server.name))
                    .ok()
                    .or(self.network
                        .as_ref()
                        .and_then(|nw| nw.servers.get(&self.server.name))
                        .map(|s| s.port.to_string()))
            },
            "SERVER_IP" => {
                std::env::var(format!("IP_{}", self.server.name))
                    .ok()
                    .or(self.network
                        .as_ref()
                        .and_then(|nw| nw.servers.get(&self.server.name))
                        .and_then(|s| s.ip_address.clone()))
            },

            "PLUGIN_COUNT" => Some(self.server.plugins.len().to_string()),
            "MOD_COUNT" => Some(self.server.mods.len().to_string()),
            "WORLD_COUNT" => Some(self.server.worlds.len().to_string()),
            "CLIENTSIDE_MOD_COUNT" => Some(self.server.clientsidemods.len().to_string()),

            "NETWORK_NAME" => Some(self.network.as_ref()?.name.clone()),
            "NETWORK_PORT" => Some(self.network.as_ref()?.port.to_string()),
            "NETWORK_SERVERS_COUNT" => Some(self.network.as_ref()?.servers.len().to_string()),

            "NETWORK_VELOCITY_SERVERS" => {
                if let Some(nw) = &self.network {
                    Some("# generated by mcman\n".to_owned() + &nw.servers.iter().map(|(name, serv)|
                        format!(
                            "{name} = \"{}:{}\"",
                            std::env::var(format!("IP_{name}")).ok()
                                .or(serv.ip_address.clone())
                                .unwrap_or("127.0.0.1".to_owned()),
                            std::env::var(format!("PORT_{name}")).ok()
                                .unwrap_or(serv.port.to_string()),
                        )
                    ).collect::<Vec<_>>().join("\n"))
                } else {
                    None
                }
            },

            "NETWORK_BUNGEECORD_SERVERS" => {
                if let Some(nw) = &self.network {
                    Some("# generated by mcman\nservers:".to_owned() + &nw.servers.iter().map(|(name, serv)|
                        format!(
                            "  {name}:\n    motd: {}\n    address: {}:{}\n    restricted: false",
                            self.var("MOTD").unwrap_or("a mcman-powered server".to_owned()),
                            std::env::var(format!("IP_{name}")).ok()
                                .or(serv.ip_address.clone())
                                .unwrap_or("127.0.0.1".to_owned()),
                            std::env::var(format!("PORT_{name}")).ok()
                                .unwrap_or(serv.port.to_string()),
                        )
                    ).collect::<Vec<_>>().join("\n"))
                } else {
                    None
                }
            },

            // why not
            "TECHNOBLADE" => Some("Technoblade never dies".to_owned()),
            "denizs_gf" => Some("ily may".to_owned()),

            k => if let Some(v) = std::env::var(k).ok() {
                Some(v)
            } else {
                if k.starts_with("NW_") {
                    if let Some(nw) = &self.network {
                        if k.starts_with("NW_SERVER_") {
                            let (name, ty) = k.strip_prefix("NW_SERVER_")
                                .unwrap()
                                .split_once('_')?;
                            
                            let serv = nw.servers.get(&name.to_lowercase())?;

                            let ip = std::env::var(format!("IP_{name}")).ok()
                                .or(serv.ip_address.clone())
                                .unwrap_or("127.0.0.1".to_owned());

                            let port = std::env::var(format!("PORT_{name}")).ok()
                                .unwrap_or(serv.port.to_string());

                            match ty.to_lowercase().as_str() {
                                "ip" => Some(ip),
                                "port" => Some(port),
                                "address" => Some(format!("{ip}:{port}")),
                                _ => None,
                            }
                        } else {
                            nw.variables.get(k.strip_prefix("NW_").unwrap()).cloned()
                        }
                    } else {
                        None
                    }
                } else {
                    self.server.variables.get(k).cloned()
                }
            }
        }
    }

    pub fn get_cache(&self, ns: &str) -> Option<Cache> {
        // TODO check if cache should be enabled to return None
        Cache::get_cache(ns)
    }
}

macro_rules! api_methods {
    ($name:ident, $t:ident) => {
        pub fn $name(&'a self) -> sources::$name::$t<'a> {
            sources::$name::$t(&self)
        }
    };
}

impl<'a> App {
    api_methods!(vanilla, VanillaAPI);

    api_methods!(github, GithubAPI);
    api_methods!(maven, MavenAPI);
    
    api_methods!(modrinth, ModrinthAPI);
    api_methods!(curserinth, CurserinthAPI);

    api_methods!(neoforge, NeoforgeAPI);
    api_methods!(forge, ForgeAPI);
    api_methods!(fabric, FabricAPI);
    api_methods!(quilt, QuiltAPI);
    
    api_methods!(papermc, PaperMCAPI);
    api_methods!(hangar, HangarAPI);
    api_methods!(purpur, PurpurAPI);
    api_methods!(spigot, SpigotAPI);

    api_methods!(mclogs, MCLogsAPI);

    pub fn markdown(&'a self) -> crate::interop::markdown::MarkdownAPI<'a> {
        crate::interop::markdown::MarkdownAPI(&self)
    }

    pub fn packwiz(&'a mut self) -> crate::interop::packwiz::PackwizInterop<'a> {
        crate::interop::packwiz::PackwizInterop(self)
    }

    pub fn mrpack(&'a mut self) -> crate::interop::mrpack::MRPackInterop<'a> {
        crate::interop::mrpack::MRPackInterop(self)
    }
}
