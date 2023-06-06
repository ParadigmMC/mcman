use console::style;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::{downloadable::Downloadable, model::Server};
use anyhow::{anyhow, bail, Result};
use clap::{arg, ArgMatches, Command};

pub fn cli() -> Command {
    Command::new("init")
        .about("Initializes a new MCMan-powered Minecraft server")
        .arg(arg!(--name <NAME> "The name of the server").required(false))
        .arg(arg!(--proxy "Initialize as a proxy server (velocity latest)").required(false))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    println!(" > {}", style("initializing server...").dim());

    let res = std::fs::metadata("server.toml");
    if let Err(err) = res {
        if err.kind() != std::io::ErrorKind::NotFound {
            Err(err)?;
        }
    } else {
        bail!("server.toml already exists");
    }

    let current_dir = std::env::current_dir()?;
    let name = matches.get_one::<String>("name");
    let name = if let Some(name) = name {
        name.clone()
    } else {
        current_dir
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or(anyhow!(
                "Can't get server name from current path; please specify a name using --name"
            ))?
            .to_owned()
    };

    let mut server = Server {
        name,
        ..Default::default()
    };

    if *matches.get_one::<bool>("proxy").unwrap_or(&false) {
        server.set_proxy_defaults();
        server.jar = Downloadable::Velocity {};
    }

    initialize_environment()?;
    server.save(Path::new("server.toml"))?;

    println!(" > {}", style("server has been initialized").green());

    Ok(())
}

pub fn initialize_environment() -> Result<()> {
    std::fs::create_dir_all("./config")?;

    let mut f = File::create(".dockerignore")?;
    f.write_all(include_bytes!("../../res/default_dockerignore"))?;

    let mut f = File::create(".gitignore")?;
    f.write_all(include_bytes!("../../res/default_gitignore"))?;

    let mut f = File::create("Dockerfile")?;
    f.write_all(include_bytes!("../../res/default_dockerfile"))?;

    let mut f = File::create("./config/server.properties")?;
    f.write_all(include_bytes!("../../res/server.properties"))?;

    Ok(())
}
