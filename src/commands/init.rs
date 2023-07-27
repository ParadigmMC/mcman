use console::style;
use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::Builder;
use zip::ZipArchive;

use crate::commands::markdown;
use crate::create_http_client;
use crate::util::mrpack::{mrpack_import_configs, mrpack_read_index, mrpack_source_to_file};
use crate::util::packwiz::{packwiz_fetch_pack_from_src, packwiz_import_from_source};
use crate::{
    downloadable::{sources::vanilla::fetch_latest_mcver, Downloadable},
    model::{Server, ServerLauncher},
};
use anyhow::{bail, Context, Result};
use clap::{arg, ArgMatches, Command};

pub fn cli() -> Command {
    Command::new("init")
        .about("Initialize a new mcman server")
        .arg(arg!(--name [NAME] "The name of the server"))
        .arg(arg!(--mrpack [SRC] "Import from a modrinth modpack"))
        .arg(arg!(--packwiz [SRC] "Import from a packwiz pack").visible_alias("pw"))
}

#[allow(clippy::too_many_lines)]
pub async fn run(matches: &ArgMatches) -> Result<()> {
    let http_client = create_http_client()?;

    println!(" > {}", style("Initializing new server...").cyan());

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
            .unwrap_or("")
            .to_owned()
    };

    let theme = ColorfulTheme::default();

    let name = Input::<String>::with_theme(&theme)
        .with_prompt("Server name?")
        .default(name.clone())
        .with_initial_text(&name)
        .interact_text()?;

    if let Some(src) = matches.get_one::<String>("mrpack") {
        init_mrpack(src, &name, &http_client).await?;
    } else if let Some(src) = matches.get_one::<String>("packwiz") {
        init_packwiz(src, &name, &http_client).await?;
    } else {
        init_normal(&http_client, &name).await?;
    }

    Ok(())
}

pub async fn init_normal(http_client: &reqwest::Client, name: &str) -> Result<()> {
    let serv_type = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Type of server?")
        .default(0)
        .items(&[
            "Normal Server (vanilla, spigot, paper etc.)",
            "Modded Server (forge, fabric, quilt etc.)",
            "Proxy Server (velocity, bungeecord, waterfall etc.)",
        ])
        .interact()?;

    let is_proxy = serv_type == 2;

    let mc_version = if is_proxy {
        "latest".to_owned()
    } else {
        let latest_ver = fetch_latest_mcver(http_client)
            .await
            .context("Fetching latest version")?;

        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Server version?")
            .default(latest_ver)
            .interact_text()?
    };

    let launcher = if is_proxy {
        ServerLauncher {
            proxy_flags: true,
            aikars_flags: false,
            nogui: false,
            ..Default::default()
        }
    } else {
        ServerLauncher::default()
    };

    let jar = match serv_type {
        0 => Downloadable::select_jar_interactive(),
        1 => Downloadable::select_modded_jar_interactive(),
        2 => Downloadable::select_proxy_jar_interactive(),
        _ => unreachable!(),
    }?;

    let server = Server {
        name: name.to_owned(),
        mc_version,
        jar,
        launcher,
        ..Default::default()
    };

    init_final(&server, is_proxy)?;

    Ok(())
}

pub async fn init_mrpack(src: &str, name: &str, http_client: &reqwest::Client) -> Result<()> {
    println!(" > {}", style("Importing from mrpack...").cyan());

    let tmp_dir = Builder::new().prefix("mcman-mrpack-import").tempdir()?;

    let mut server = Server {
        name: name.to_owned(),
        ..Default::default()
    };

    let f = mrpack_source_to_file(src, http_client, &tmp_dir, &server).await?;

    let mut archive = ZipArchive::new(f).context("reading mrpack zip archive")?;

    let pack = mrpack_read_index(&mut archive)?;

    server.mc_version = if let Some(v) = pack.dependencies.get("minecraft") {
        v.clone()
    } else {
        let latest_ver = fetch_latest_mcver(http_client)
            .await
            .context("Fetching latest version")?;

        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Server version?")
            .default(latest_ver)
            .interact_text()?
    };

    server.jar = {
        if let Some(ver) = pack.dependencies.get("quilt-loader") {
            println!(
                " > {} {}",
                style("Using quilt loader").cyan(),
                style(ver).bold()
            );
            Downloadable::Quilt {
                loader: ver.clone(),
                installer: "latest".to_owned(),
            }
        } else if let Some(ver) = pack.dependencies.get("fabric-loader") {
            println!(
                " > {} {}",
                style("Using fabric loader").cyan(),
                style(ver).bold()
            );
            Downloadable::Fabric {
                loader: ver.clone(),
                installer: "latest".to_owned(),
            }
        } else {
            Downloadable::select_modded_jar_interactive()?
        }
    };

    println!(" > {}", style("Importing mods...").green().bold());

    pack.import_all(&mut server, http_client).await?;

    println!(
        " > {}",
        style("Importing configuration files...").green().bold()
    );

    let config_count = mrpack_import_configs(&server, &mut archive)?;

    println!(
        " > {} {} {} {} {}",
        style("Imported").green().bold(),
        style(pack.files.len()).cyan(),
        style("mods and").green(),
        style(config_count).cyan(),
        style("config files from .mrpack").green(),
    );

    init_final(&server, false)?;

    Ok(())
}

pub async fn init_packwiz(src: &str, name: &str, http_client: &reqwest::Client) -> Result<()> {
    println!(" > {}", style("Importing from packwiz...").cyan());

    let mut server = Server {
        name: name.to_owned(),
        ..Default::default()
    };

    let pack = packwiz_fetch_pack_from_src(http_client, src).await?;

    server.mc_version = if let Some(v) = pack.versions.get("minecraft") {
        v.clone()
    } else {
        // TODO: [acceptable-versions] or something idk

        let latest_ver = fetch_latest_mcver(http_client)
            .await
            .context("Fetching latest version")?;

        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Server version?")
            .default(latest_ver)
            .interact_text()?
    };

    server.jar = {
        if let Some(ver) = pack.versions.get("quilt") {
            println!(
                " > {} {}",
                style("Using quilt loader").cyan(),
                style(ver).bold()
            );
            Downloadable::Quilt {
                loader: ver.clone(),
                installer: "latest".to_owned(),
            }
        } else if let Some(ver) = pack.versions.get("fabric") {
            println!(
                " > {} {}",
                style("Using fabric loader").cyan(),
                style(ver).bold()
            );
            Downloadable::Fabric {
                loader: ver.clone(),
                installer: "latest".to_owned(),
            }
        } else {
            Downloadable::select_modded_jar_interactive()?
        }
    };

    let (_pack, mod_count, config_count) =
        packwiz_import_from_source(http_client, src, &mut server).await?;

    println!(
        " > {} {} {} {} {}",
        style("Imported").green().bold(),
        style(mod_count).cyan(),
        style("mods and").green(),
        style(config_count).cyan(),
        style("config files from packwiz").green(),
    );

    init_final(&server, false)?;

    Ok(())
}

pub fn init_final(server: &Server, is_proxy: bool) -> Result<()> {
    server.save()?;

    initialize_environment(is_proxy).context("Initializing environment")?;

    let write_readme = if Path::new("./README.md").exists() {
        Confirm::with_theme(&ColorfulTheme::default())
            .default(true)
            .with_prompt("Overwrite README.md?")
            .interact()?
    } else {
        true
    };

    if write_readme {
        markdown::initialize_readme(server).context("Initializing readme")?;
    }

    println!(
        " > {} {}",
        style(&server.name).bold(),
        style("has been initialized!").green()
    );

    println!(
        " > {} {}",
        style("Build using").cyan(),
        style("mcman build").bold()
    );

    Ok(())
}

pub fn initialize_environment(is_proxy: bool) -> Result<()> {
    std::fs::create_dir_all("./config")?;

    let mut f = File::create(".dockerignore")?;
    f.write_all(include_bytes!("../../res/default_dockerignore"))?;

    let mut f = File::create(".gitignore")?;
    f.write_all(include_bytes!("../../res/default_gitignore"))?;

    let mut f = File::create("Dockerfile")?;
    f.write_all(include_bytes!("../../res/default_dockerfile"))?;

    if !is_proxy {
        let mut f = File::create("./config/server.properties")?;
        f.write_all(include_bytes!("../../res/server.properties"))?;
    }

    Ok(())
}
