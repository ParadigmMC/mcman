use std::{
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
    pub disable: bool,
    pub jvm_args: String,
    pub game_args: String,
    pub aikars_flags: bool,
    pub proxy_flags: bool,
    pub gui: bool,
    pub memory: u8,
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

        // todo: custom java stuff from ~/.mcmanconfig or something idk
        script.push_str("java ");

        if self.memory > 0 {
            script.push_str("-Xms");
        }

        if self.aikars_flags {
            script += include_str!("../../res/aikars_flags");
            script += " ";
        }

        if self.proxy_flags {
            script += include_str!("../../res/proxy_flags");
            script += " ";
        }

        script += "-jar ";
        script += jarname;
        script += " ";

        if !self.gui {
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
}

impl Default for Server {
    fn default() -> Self {
        Self {
            name: String::new(),
            mc_version: "1.19.4".to_owned(),
            launcher: ServerLauncher::default(),
            jar: Downloadable::Vanilla {
                version: "1.19.4".to_owned(),
            },
            plugins: vec![],
        }
    }
}
