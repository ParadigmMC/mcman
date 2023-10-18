use console::style;
use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, Input};
use indicatif::{MultiProgress, ProgressBar};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::Builder;

use crate::commands::init::init::init_normal;
use crate::commands::init::network::init_network;
use crate::commands::markdown;
use crate::app::{BaseApp, App};
use crate::model::Network;
use crate::util::env::{get_docker_version, write_dockerfile, write_dockerignore, write_gitignore};
use crate::model::Server;
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
            multi_progress: MultiProgress::new(),
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
                name: name.clone(),
                ..Default::default()
            },
            multi_progress: MultiProgress::new(),
        };

        if let Some(src) = args.mrpack {
            let tmp_dir = Builder::new().prefix("mcman-mrpack-import").tempdir()?;

            let f = if Path::new(&src).exists() {
                std::fs::File::open(&src)?
            } else {
                let dl = app.dl_from_string(&src).await?;
                let resolved = app.download(&dl, tmp_dir.path().to_path_buf(), ProgressBar::new_spinner()).await?;
                let path = tmp_dir.path().join(&resolved.filename);
                std::fs::File::open(path)?
            };
            app.mrpack().import_all(f, Some(name)).await?;
        } else if let Some(src) = args.packwiz {
            app.packwiz().import_all(&src).await?;
        } else {
            init_normal(&mut app).await?;
        }
    }

    Ok(())
}

pub async fn init_final(
    app: &App,
    server: &mut Server,
    is_proxy: bool,
) -> Result<()> {
    app.server.save()?;

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

        app.markdown().update_files().await?;
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
        f.write_all(include_bytes!("../../../res/server.properties"))?;
    }

    Ok(())
}
