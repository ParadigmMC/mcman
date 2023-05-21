use std::path::Path;
use std::{ffi::OsStr, path::PathBuf};

use crate::{downloadable::Downloadable, model::Server};
use anyhow::{Context, Result};
use clap::Command;
use console::Style;
use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn cli() -> Command {
    Command::new("setup").about("Interactive setup")
}

struct SetupContext {
    pub server: Server,
    pub theme: ColorfulTheme,
    pub first_run: bool,
    pub current_dir: PathBuf,
}

pub fn run() -> Result<()> {
    let mut server = Server {
        ..Default::default()
    };
    let mut first_run = true;

    let res = std::fs::metadata("server.toml");
    if let Err(err) = res {
        if err.kind() == std::io::ErrorKind::NotFound {
            Err(err)?;
        }
    } else {
        server = Server::load(Path::new("server.toml")).context("Failed to load server.toml")?;
        first_run = false;
    }

    let current_dir = std::env::current_dir()?;

    println!(" [WARNING] SETUP IS EXPERIMENTAL");

    if first_run {
        println!(" Initializing a new server...");
    }

    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    let mut ctx = SetupContext {
        server,
        theme,
        first_run,
        current_dir,
    };

    if first_run {
        pick_server_name(&mut ctx)?;
        pick_server_version(&mut ctx)?;
        pick_server_jar(&mut ctx)?;
    }

    options_loop(&mut ctx)?;

    Ok(())
}

fn options_loop(ctx: &mut SetupContext) -> Result<()> {
    loop {
        let option_sel = Select::with_theme(&ctx.theme)
            .with_prompt("What would you like to do?")
            .default(0)
            .item("Set name")
            .item("Set mc version")
            .item("Set jar")
            .item("Exit")
            .interact()?;

        match option_sel {
            0 => pick_server_name(ctx),
            1 => pick_server_version(ctx),
            2 => pick_server_jar(ctx),
            3 => {
                if Confirm::with_theme(&ctx.theme)
                    .with_prompt("Save configuration?")
                    .interact()?
                {
                    println!("Saving...");
                    std::fs::create_dir_all("./config")?;
                    ctx.server.save(Path::new("server.toml"))?;
                } else {
                    println!(" -> Cancelled");
                }
                break;
            }
            _ => break,
        }?;
    }

    Ok(())
}

fn pick_server_name(ctx: &mut SetupContext) -> Result<()> {
    let mut def_serv_name = ctx.server.name.clone();

    if def_serv_name.is_empty() {
        def_serv_name = ctx
            .current_dir
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            .to_owned();
    }

    let server_name = Input::with_theme(&ctx.theme)
        .with_prompt("Name")
        .with_initial_text(def_serv_name)
        .interact()?;
    ctx.server.name = server_name;

    Ok(())
}

/* fn pick_server_port(ctx: &mut SetupContext) -> Result<()> {
    let server_port = Input::with_theme(&ctx.theme)
        .with_prompt("Port")
        .with_initial_text(ctx.server.port.to_string())
        .interact()?;
    ctx.server.port = server_port;

    Ok(())
} */

fn pick_server_version(ctx: &mut SetupContext) -> Result<()> {
    let server_ver = Input::with_theme(&ctx.theme)
        .with_prompt("MC Version")
        .with_initial_text(ctx.server.mc_version.clone())
        .interact()?;
    ctx.server.mc_version = server_ver;

    Ok(())
}

fn pick_server_jar(ctx: &mut SetupContext) -> Result<()> {
    let def_jar_id = match ctx.server.jar {
        Downloadable::Vanilla {} => 0,
        Downloadable::Paper {} => 1,
        Downloadable::Folia {} => 2,
        Downloadable::Purpur {
            build: _,
        } => 3,
        Downloadable::Velocity {} => 4,
        Downloadable::Waterfall {} => 5,
        _ => 6,
    };

    let server_jar_type = Select::with_theme(&ctx.theme)
        .with_prompt("Server Jar Type")
        .default(def_jar_id)
        .item("Vanilla")
        .item("Paper")
        .item("Folia")
        .item("Purpur")
        .item("(proxy) Velocity")
        .item("(proxy) Waterfall")
        .item("-> From Custom URL")
        .interact()?;

    let jar_dl = match server_jar_type {
        0 => Downloadable::Vanilla {},
        1 => Downloadable::Paper {},
        2 => Downloadable::Folia {},
        3 => Downloadable::Purpur { build: "latest".to_owned() },
        4 => Downloadable::Velocity {},
        5 => Downloadable::Waterfall {},
        6 => Downloadable::Url {
            url: Input::with_theme(&ctx.theme)
                .with_prompt("Server Jar URL")
                .interact()?,
        },
        _ => Downloadable::Url { url: String::new() },
    };

    ctx.server.jar = jar_dl;

    Ok(())
}
