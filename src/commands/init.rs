use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::error::{CliError, Result};
use crate::model::Server;
use clap::{arg, ArgMatches, Command};

pub fn cli() -> Command {
    Command::new("init")
        .about("Initializes a new MCMan-powered Minecraft server")
        .arg(arg!(--name <NAME> "The name of the server").required(false))
}

pub fn run(matches: &ArgMatches) -> Result<()> {
    let res = std::fs::metadata("server.toml");
    if let Err(err) = res {
        if err.kind() != std::io::ErrorKind::NotFound {
            Err(err)?;
        }
    } else {
        Err(CliError::AlreadyExists)?;
    }

    let current_dir = std::env::current_dir()?;
    let name = matches.get_one::<String>("name");
    let name = if let Some(name) = name {
        name.clone()
    } else {
        current_dir
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or(CliError::MissingServerName)?
            .to_owned()
    };

    let server = Server {
        name,
        ..Default::default()
    };

    std::fs::create_dir_all("./config")?;
    server.save(Path::new("server.toml"))?;

    let mut f = File::create(".gitignore")?;
    f.write_all(include_bytes!("../../res/default_gitignore"))?;

    Ok(())
}
