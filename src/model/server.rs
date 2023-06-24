use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::downloadable::Downloadable;

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ServerLauncher {
    pub aikars_flags: bool,
    pub proxy_flags: bool,
    pub nogui: bool,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub disable: bool,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub jvm_args: String,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub game_args: String,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub eula_args: bool,
    #[serde(skip_serializing_if = "crate::util::is_default")]
    pub memory: String,
}

impl ServerLauncher {
    pub fn generate_script_linux(&mut self, jarname: &str, servername: &str) -> String {
        format!(
            "#!/bin/sh\n{}\n",
            self.generate_script_java(jarname, servername)
        )
    }

    pub fn generate_script_win(&mut self, jarname: &str, servername: &str) -> String {
        format!(
            "@echo off\ntitle {servername}\n{}\n",
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

    #[allow(dead_code)]
    pub fn format(&self, str: &str) -> String {
        mcapi::dollar_repl(str, |key| {
            match key {
                "mcver" | "mcversion" => Some(self.mc_version.clone()),
                // Maybe also allow self.variables? idk
                _ => None,
            }
        })
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
