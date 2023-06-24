use std::{
    collections::HashMap,
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};

use anyhow::{Context, Result};
use clap::{arg, value_parser, ArgMatches, Command};
use console::{style, Style};
use tokio::fs::File;

use super::version::APP_USER_AGENT;
use crate::{
    bootstrapper::{bootstrap, BootstrapContext},
    downloadable::Downloadable,
    model::Server,
    util,
};

pub fn cli() -> Command {
    Command::new("build")
        .about("Build using server.toml configuration")
        .arg(
            arg!(-o --output <FILE> "The output directory for the server")
                .default_value("server")
                .value_parser(value_parser!(PathBuf)),
        )
}

pub async fn run(matches: &ArgMatches) -> Result<()> {
    let mut server =
        Server::load(Path::new("server.toml")).context("Failed to load server.toml")?;
    let http_client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .context("Failed to create HTTP client")?;
    let output_dir = matches.get_one::<PathBuf>("output").unwrap();
    std::fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    //let term = Term::stdout();
    let title = Style::new().blue().bold();

    let start_time = Instant::now();

    println!(" Building {}...", style(server.name.clone()).green().bold());

    let mut stage_index = 1;

    let mut mark_stage = |stage_name| {
        println!(" stage {stage_index}: {}", title.apply_to(stage_name));
        stage_index += 1;
    };

    // stage 1: server jar
    mark_stage("Server Jar");

    let serverjar_name = download_server_jar(&server, &http_client, output_dir)
        .await
        .context("Failed to download server jar")?;

    // stage 2: plugins
    if !server.plugins.is_empty() {
        mark_stage("Plugins");
        download_addons("plugins", &server, &http_client, output_dir)
            .await
            .context("Failed to download plugins")?;
    }

    // stage 3: mods
    if !server.mods.is_empty() {
        mark_stage("Mods");
        download_addons("mods", &server, &http_client, output_dir)
            .await
            .context("Failed to download plugins")?;
    }

    // stage 4: bootstrap

    mark_stage("Configurations");

    let mut vars = HashMap::new();

    for (key, value) in &server.variables {
        vars.insert(key.clone(), value.clone());
    }

    for (key, value) in env::vars() {
        vars.insert(key.clone(), value.clone());
    }

    vars.insert("SERVER_NAME".to_owned(), server.name.clone());
    vars.insert("SERVER_VERSION".to_owned(), server.mc_version.clone());

    bootstrap(
        &BootstrapContext {
            vars,
            output_dir: output_dir.clone(),
        },
        "config",
    )?;

    println!("          {}", style("Bootstrapping complete").dim());

    // stage 5: launcher scripts
    if !server.launcher.disable {
        mark_stage("Scripts");

        fs::write(
            output_dir.join("start.bat"),
            server
                .launcher
                .generate_script_win(&serverjar_name.clone(), &server.name),
        )?;

        let mut file;
        #[cfg(target_family = "unix")]
        {
            use std::os::unix::prelude::OpenOptionsExt;
            file = OpenOptions::new()
                .create(true)
                .write(true)
                .mode(0o755)
                .open(output_dir.join("start.sh"))?;
        }
        #[cfg(not(target_family = "unix"))]
        {
            file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(output_dir.join("start.sh"))?;
        }

        file.write_all(
            server
                .launcher
                .generate_script_linux(&serverjar_name, &server.name)
                .as_bytes(),
        )?;

        println!(
            "          {}",
            style("start.bat and start.sh created").dim()
        );
    }

    println!(
        " Successfully built {} in {}",
        style(server.name.clone()).green().bold(),
        style(start_time.elapsed().as_millis().to_string() + "ms").blue(),
    );

    Ok(())
}

async fn download_server_jar(
    server: &Server,
    http_client: &reqwest::Client,
    output_dir: &Path,
) -> Result<String> {
    let serverjar_name = server.jar.get_filename(server, http_client).await?;
    if output_dir.join(serverjar_name.clone()).exists() {
        println!(
            "          Skipping server jar ({})",
            style(serverjar_name.clone()).dim()
        );
    } else {
        println!(
            "          Downloading server jar ({})",
            style(serverjar_name.clone()).dim()
        );

        let filename = &server.jar.get_filename(server, http_client).await?;
        util::download_with_progress(
            File::create(&output_dir.join(filename))
                .await
                .context(format!("Failed to create output file for {filename}"))?,
            filename,
            &server.jar,
            server,
            http_client,
        )
        .await?;

        match &server.jar {
            Downloadable::Quilt { .. } | Downloadable::Fabric { .. } => {
                todo!()
            }
            _ => (),
        }
    }

    Ok(serverjar_name)
}

async fn download_addons(
    addon_type: &str,
    server: &Server,
    http_client: &reqwest::Client,
    output_dir: &Path,
) -> Result<()> {
    let addon_count = match addon_type {
        "plugins" => server.plugins.len(),
        "mods" => server.mods.len(),
        _ => unreachable!(),
    };

    println!(
        "          {}",
        style(format!("{addon_count} {addon_type} present, processing...")).dim()
    );

    std::fs::create_dir_all(output_dir.join(addon_type))
        .context(format!("Failed to create {addon_type} directory"))?;

    let mut i = 0;
    for addon in match addon_type {
        "plugins" => &server.plugins,
        "mods" => &server.mods,
        _ => unreachable!(),
    } {
        i += 1;

        let addon_name = addon.get_filename(server, http_client).await?;
        if output_dir.join(addon_type).join(&addon_name).exists() {
            println!(
                "          ({}/{}) Skipping    : {}",
                i,
                addon_count,
                style(&addon_name).dim()
            );
            continue;
        }

        util::download_with_progress(
            File::create(&output_dir.join(addon_type).join(addon_name.clone()))
                .await
                .context(format!("Failed to create output file for {addon_name}"))?,
            &addon_name,
            addon,
            server,
            http_client,
        )
        .await
        .context(format!("Failed to download plugin {addon_name}"))?;

        println!(
            "          ({}/{}) {}  : {}",
            i,
            addon_count,
            style("Downloaded").green().bold(),
            style(&addon_name).dim()
        );
    }

    Ok(())
}
