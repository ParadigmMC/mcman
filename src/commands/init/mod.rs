use console::style;
use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::Builder;
use zip::ZipArchive;

use crate::commands::init::init::init_normal;
use crate::commands::init::network::init_network;
use crate::commands::markdown;
use crate::{BaseApp, App};
use crate::model::{Network, ServerType};
use crate::util::env::{get_docker_version, write_dockerfile, write_dockerignore, write_gitignore};
use crate::util::mrpack::{mrpack_import_configs, mrpack_read_index, mrpack_source_to_file};
use crate::util::packwiz::{packwiz_fetch_pack_from_src, packwiz_import_from_source};
use crate::model::{Server, ServerLauncher};
use anyhow::{bail, Context, Result};

pub mod init;
pub mod network;
pub mod packwiz;
pub mod mrpack;

#[derive(clap::Args)]
pub struct Args {
    /// The name of the server
    #[arg(long)]
    name: Option<String>,
    /// Import from a modrinth modpack
    #[arg(long, visible_alias = "mr", value_name = "src", group = "type")]
    mrpack: Option<String>,
    /// Import from a packwiz pack
    #[arg(long, visible_alias = "pw", value_name = "src", group = "type")]
    packwiz: Option<String>,
    /// Create a default network.toml
    #[arg(long, visible_alias = "nw", group = "type")]
    network: bool,
}

#[allow(dead_code)]
pub enum InitType {
    Normal,
    MRPack(String),
    Packwiz(String),
    Network,
}

#[allow(clippy::too_many_lines)]
pub async fn run(base_app: BaseApp, args: Args) -> Result<()> {
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
    let name = if let Some(name) = args.name {
        name.clone()
    } else {
        current_dir
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("")
            .to_owned()
    };

    let theme = ColorfulTheme::default();

    if args.network {
        let name = Input::<String>::with_theme(&theme)
            .with_prompt("Network name?")
            .default(name.clone())
            .with_initial_text(&name)
            .interact_text()?;

        let mut app = App {
            http_client: base_app.http_client,
            network: Some(Network {
                name,
                ..Default::default()
            }),
            server: Server::default(),
        };

        init_network(&mut app).await?;
    } else {
        let name = Input::<String>::with_theme(&theme)
            .with_prompt("Server name?")
            .default(name.clone())
            .with_initial_text(&name)
            .interact_text()?;

        let mut app = App {
            http_client: base_app.http_client,
            network: None,
            server: Server {
                name,
                ..Default::default()
            },
        };

        if let Some(src) = args.mrpack {
            init_mrpack(&src, &name, &mut app).await?;
        } else if let Some(src) = args.packwiz {
            init_packwiz(&src, &name, &mut app).await?;
        } else {
            init_normal(&mut app).await?;
        }
    }

    Ok(())
}


pub async fn init_mrpack(src: &str, name: &str, app: &BaseApp) -> Result<()> {
    println!(" > {}", style("Importing from mrpack...").cyan());

    let tmp_dir = Builder::new().prefix("mcman-mrpack-import").tempdir()?;

    let mut server = Server {
        name: name.to_owned(),
        ..Default::default()
    };

    let f = mrpack_source_to_file(src, app, &tmp_dir, &server).await?;

    let mut archive = ZipArchive::new(f).context("reading mrpack zip archive")?;

    let pack = mrpack_read_index(&mut archive)?;

    server.mc_version = if let Some(v) = pack.dependencies.get("minecraft") {
        v.clone()
    } else {
        let latest_ver = fetch_latest_mcver(app)
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
            ServerType::Quilt {
                loader: ver.clone(),
                installer: "latest".to_owned(),
            }
        } else if let Some(ver) = pack.dependencies.get("fabric-loader") {
            println!(
                " > {} {}",
                style("Using fabric loader").cyan(),
                style(ver).bold()
            );
            ServerType::Fabric {
                loader: ver.clone(),
                installer: "latest".to_owned(),
            }
        } else {
            ServerType::select_modded_jar_interactive()?
        }
    };

    println!(" > {}", style("Importing mods...").green().bold());

    pack.import_all(&mut server, app).await?;

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

    init_final(app, &mut server, false).await?;

    Ok(())
}

pub async fn init_packwiz(src: &str, name: &str, app: &BaseApp) -> Result<()> {
    println!(" > {}", style("Importing from packwiz...").cyan());

    let mut server = Server {
        name: name.to_owned(),
        ..Default::default()
    };

    let pack = packwiz_fetch_pack_from_src(app, src).await?;

    server.mc_version = if let Some(v) = pack.versions.get("minecraft") {
        v.clone()
    } else {
        // TODO: [acceptable-versions] or something idk

        let latest_ver = fetch_latest_mcver(app)
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
            ServerType::Quilt {
                loader: ver.clone(),
                installer: "latest".to_owned(),
            }
        } else if let Some(ver) = pack.versions.get("fabric") {
            println!(
                " > {} {}",
                style("Using fabric loader").cyan(),
                style(ver).bold()
            );
            ServerType::Fabric {
                loader: ver.clone(),
                installer: "latest".to_owned(),
            }
        } else {
            ServerType::select_modded_jar_interactive()?
        }
    };

    let (_pack, mod_count, config_count) =
        packwiz_import_from_source(app, src, &mut server).await?;

    println!(
        " > {} {} {} {} {}",
        style("Imported").green().bold(),
        style(mod_count).cyan(),
        style("mods and").green(),
        style(config_count).cyan(),
        style("config files from packwiz").green(),
    );

    init_final(app, &mut server, false).await?;

    Ok(())
}

pub async fn init_final(
    app: &BaseApp,
    server: &mut Server,
    is_proxy: bool,
) -> Result<()> {
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

        server.markdown.files = vec!["README.md".to_owned()];
        server.save()?;

        super::markdown::update_files(app, server)
            .await
            .context("updating markdown files")?;
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

    let theme = ColorfulTheme::default();

    if write_gitignore().is_err() {
        println!(
            "{} {}{}{}",
            theme.prompt_prefix,
            style("Didn't find a git repo, use '").dim(),
            style("mcman env gitignore").bold(),
            style("' to generate .gitignore").dim(),
        );
    } else {
        println!(
            "{} {}",
            theme.success_prefix,
            style("Touched up .gitignore").dim(),
        );
    }

    if let Ok(Some(_)) = get_docker_version() {
        write_dockerfile(Path::new("."))?;
        write_dockerignore(Path::new("."))?;
        println!(
            "{} {}",
            theme.success_prefix,
            style("Docker files were written").dim(),
        );
    } else {
        println!(
            "{} {}{}{}",
            theme.prompt_prefix,
            style("Docker wasn't found, you can use '").dim(),
            style("mcman env docker").bold(),
            style("' to generate docker files").dim(),
        );
    }

    if !is_proxy {
        let mut f = File::create("./config/server.properties")?;
        f.write_all(include_bytes!("../../res/server.properties"))?;
    }

    Ok(())
}
