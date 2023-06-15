use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use anyhow::{anyhow, Result};
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::downloadable::Downloadable;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ServerLauncher {
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub disable: bool,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub jvm_args: String,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub game_args: String,
    pub aikars_flags: bool,
    pub proxy_flags: bool,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub eula_args: bool,
    pub nogui: bool,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub memory: String,
}

impl ServerLauncher {
    pub fn generate_script_linux(&mut self, jarname: &str, servername: &str) -> String {
        format!(
            "#!/bin/sh
{}
",
            self.generate_script_java(jarname, servername)
        )
    }

    pub fn generate_script_win(&mut self, jarname: &str, servername: &str) -> String {
        format!(
            "@echo off
title {servername}
{}
",
            self.generate_script_java(jarname, servername)
        )
    }

    pub fn generate_script_java(&mut self, jarname: &str, _servername: &str) -> String {
        let mut script = String::new();

        // TODO: custom java stuff from ~/.mcmanconfig or something idk
        script.push_str("java ");

        if !self.jvm_args.is_empty() {
            script += &self.jvm_args.to_string();
            script += " ";
        }

        if !self.memory.is_empty() {
            script += "-Xms";
            script += &self.memory.to_string();
            script += " -Xmx";
            script += &self.memory.to_string();
            script += " ";
        }

        if self.aikars_flags {
            script += include_str!("../../res/aikars_flags");
            script += " ";
        }

        if self.proxy_flags {
            script += include_str!("../../res/proxy_flags");
            script += " ";
        }

        if self.eula_args {
            script += "-Dcom.mojang.eula.agree=true ";
        }

        script += "-jar ";
        script += jarname;
        script += " ";

        if self.nogui {
            script.push_str("--nogui ");
        }

        script += &self.game_args;

        script.trim().to_owned()
    }
}

impl Default for ServerLauncher {
    fn default() -> Self {
        Self {
            disable: false,
            jvm_args: String::new(),
            game_args: String::new(),
            aikars_flags: true,
            proxy_flags: false,
            nogui: true,
            eula_args: true,
            memory: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Server {
    pub name: String,
    pub mc_version: String, // TODO: version type for comparing
    pub jar: Downloadable,
    pub variables: HashMap<String, String>,
    pub launcher: ServerLauncher,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<Downloadable>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mods: Vec<Downloadable>,
}

impl Server {
    pub fn load(path: &Path) -> Result<Self> {
        let data = read_to_string(path)?;
        Ok(toml::from_str(&data)?)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let cfg_str = toml::to_string_pretty(&self)?;
        let mut f = File::create(path)?;
        f.write_all(cfg_str.as_bytes())?;

        Ok(())
    }

    pub fn set_proxy_defaults(&mut self) {
        self.launcher.proxy_flags = true;
        self.launcher.aikars_flags = false;
        self.launcher.nogui = false;
    }

    pub fn import_from_url(&mut self, urlstr: &str, is_mod: Option<bool>) -> Result<()> {
        let url = Url::parse(urlstr)?;
        match url.domain() {
            Some("cdn.modrinth.com") => {
                self.import_from_modrinthcdn(&url, Some(true))?;
            }
            Some("www.spigotmc.org") => {
                // https://www.spigotmc.org/resources/http-requests.101253/

                let segments: Vec<&str> = url
                    .path_segments()
                    .ok_or_else(|| anyhow!("Invalid url"))?
                    .collect();

                if segments.first() != Some(&"resources") {
                    Err(anyhow!("Invalid Spigot Resource URL"))?;
                }

                let id = segments
                    .get(1)
                    .ok_or_else(|| anyhow!("Invalid Spigot Resource URL"))?;

                match is_mod {
                    Some(true) => self.mods.push(Downloadable::Spigot {
                        id: id.to_owned().to_owned(),
                    }),
                    Some(false) | None => self.plugins.push(Downloadable::Spigot {
                        id: id.to_owned().to_owned(),
                    }),
                }
            }
            Some(_) | None => match is_mod {
                Some(true) => self.mods.push(Downloadable::Url {
                    url: url.to_string(),
                    filename: None,
                }),
                Some(false) | None => self.plugins.push(Downloadable::Url {
                    url: url.to_string(),
                    filename: None,
                }),
            },
        }
        Ok(())
    }

    pub fn import_from_modrinthcdn(&mut self, url: &Url, is_mod: Option<bool>) -> Result<()> {
        // https://cdn.modrinth.com/data/{ID}/versions/{VERSION}/{FILENAME}
        let segments: Vec<&str> = url
            .path_segments()
            .ok_or_else(|| anyhow!("Invalid Modrinth CDN URL"))?
            .collect();
        let id = segments
            .get(1)
            .ok_or_else(|| anyhow!("Invalid Modrinth CDN URL"))?;
        let version = segments
            .get(3)
            .ok_or_else(|| anyhow!("Invalid Modrinth CDN URL"))?;
        //let filename = segments.get(4).ok_or_else(|| anyhow!("Invalid Modrinth CDN URL"))?;

        if segments.first() != Some(&"data") || segments.get(2) != Some(&"versions") {
            Err(anyhow!("Invalid Modrinth CDN URL"))?;
        }

        match is_mod {
            Some(true) | None => self.mods.push(Downloadable::Modrinth {
                id: id.to_owned().to_owned(),
                version: version.to_owned().to_owned(),
            }),
            Some(false) => self.plugins.push(Downloadable::Modrinth {
                id: id.to_owned().to_owned(),
                version: version.to_owned().to_owned(),
            }),
        }

        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        let mut vars = HashMap::new();
        vars.insert("PORT".to_owned(), "25565".to_owned());
        Self {
            name: String::new(),
            mc_version: "latest".to_owned(),
            jar: Downloadable::Vanilla {},
            variables: vars,
            launcher: ServerLauncher::default(),
            plugins: vec![],
            mods: vec![],
        }
    }
}
