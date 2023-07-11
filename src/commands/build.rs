use std::{
    collections::HashMap,
    env,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::Stdio,
    time::{Duration, Instant},
};

use anyhow::{bail, Context, Result};
use clap::{arg, value_parser, ArgMatches, Command};
use console::{style, Style};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{fs::File, io::AsyncWriteExt};

use super::version::APP_USER_AGENT;
use crate::{
    bootstrapper::{bootstrap, BootstrapContext},
    downloadable::{sources::quilt::map_quilt_loader_version, Downloadable},
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
        .arg(
            arg!(--skip [stages] "Skip some stages")
                .value_delimiter(',')
                .default_value(""),
        )
}

#[allow(clippy::if_not_else)]
pub async fn run(matches: &ArgMatches) -> Result<()> {
    let server = Server::load().context("Failed to load server.toml")?;
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

    let skip_stages = matches
        .get_many::<String>("skip")
        .unwrap()
        .collect::<Vec<&String>>();

    let mut stage_index = 1;

    let mut mark_stage = |stage_name| {
        println!(" stage {stage_index}: {}", title.apply_to(stage_name));
        stage_index += 1;
    };

    let mark_stage_skipped = |id| {
        println!("      {}{id}", style("-> Skipping stage ").yellow().bold());
    };

    // stage 1: server jar
    mark_stage("Server Jar");

    let serverjar_name = download_server_jar(&server, &http_client, output_dir)
        .await
        .context("Failed to download server jar")?;

    // stage 2: plugins
    if !skip_stages.contains(&&"addons".to_owned()) {
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
    } else {
        mark_stage_skipped("addons");
    }

    // stage 4: bootstrap
    mark_stage("Configurations");

    if !skip_stages.contains(&&"bootstrap".to_owned()) {
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

        if server.launcher.eula_args {
            match server.jar {
                Downloadable::Quilt { .. } | Downloadable::Fabric { .. } => {
                    println!(
                        "          {}",
                        style("=> eula.txt [eula_args unsupported]").dim()
                    );
                    std::fs::File::create(output_dir.join("eula.txt"))?.write_all(b"eula=true")?;
                }
                _ => (),
            }
        }

        println!("          {}", style("Bootstrapping complete").dim());
    } else {
        mark_stage_skipped("bootstrap");
    }

    // stage 5: launcher scripts
    if !skip_stages.contains(&&"scripts".to_owned()) {
        if !server.launcher.disable {
            mark_stage("Scripts");
            create_scripts(&server, &serverjar_name, output_dir)?;
        }
    } else {
        mark_stage_skipped("scripts");
    }

    println!(
        " Successfully built {} in {}",
        style(server.name.clone()).green().bold(),
        style(start_time.elapsed().as_millis().to_string() + "ms").blue(),
    );

    Ok(())
}

async fn execute_child(
    cmd: &mut std::process::Command,
    output_dir: &Path,
    label: &str,
    tag: &str,
) -> Result<()> {
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .context("Running ".to_owned() + label)?;

    let spinner = ProgressBar::new_spinner().with_style(ProgressStyle::with_template(
        "          {spinner:.dim.bold} {msg}",
    )?);

    spinner.enable_steady_tick(Duration::from_millis(200));

    let prefix = style(format!("[{tag}]")).bold();

    let mut log_file = File::create(output_dir.join(".".to_owned() + tag + ".mcman.log")).await?;

    log_file
        .write_all(format!("=== mcman {tag} / {label} output ===").as_bytes())
        .await?;

    for buf in BufReader::new(child.stdout.take().unwrap()).lines() {
        let buf = buf.unwrap();
        let buf = buf.trim();

        if !buf.is_empty() {
            log_file.write_all(buf.as_bytes()).await?;
            log_file.write_all(b"\n").await?;

            if let Some(last_line) = buf.split('\n').last() {
                spinner.set_message(format!("{prefix} {last_line}"));
            }
        }
    }

    if !child.wait()?.success() {
        bail!("{label} exited with non-zero code");
    }

    spinner.finish_and_clear();

    Ok(())
}

#[allow(clippy::too_many_lines)]
async fn download_server_jar(
    server: &Server,
    http_client: &reqwest::Client,
    output_dir: &Path,
) -> Result<String> {
    let serverjar_name = match &server.jar {
        Downloadable::Quilt { loader, .. } => {
            let installerjar_name = server.jar.get_filename(server, http_client).await?;
            if output_dir.join(installerjar_name.clone()).exists() {
                println!(
                    "          Quilt installer present ({})",
                    style(installerjar_name.clone()).dim()
                );
            } else {
                println!(
                    "          Downloading quilt installer... ({})",
                    style(installerjar_name.clone()).dim()
                );

                let filename = &installerjar_name;
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
            }

            let loader_id = map_quilt_loader_version(http_client, loader)
                .await
                .context("getting loader version id")?;

            let serverjar_name =
                format!("quilt-server-launch-{}-{}.jar", server.mc_version, loader_id);

            if output_dir.join(serverjar_name.clone()).exists() {
                println!(
                    "          Skipping server jar ({})",
                    style(serverjar_name.clone()).dim()
                );
            } else {
                println!(
                    "          Installing quilt server... ({})",
                    style(serverjar_name.clone()).dim()
                );

                let mut args = vec![
                    "-jar",
                    &installerjar_name,
                    "install",
                    "server",
                    &server.mc_version,
                ];

                if loader != "latest" {
                    args.push(&loader);
                }

                args.push("--install-dir=.");
                args.push("--download-server");

                execute_child(
                    std::process::Command::new("java")
                        .args(args)
                        .current_dir(output_dir),
                    output_dir,
                    "Quilt server installer",
                    "qsi",
                )
                .await
                .context("Running quilt-server-installer")?;

                println!(
                    "          Renaming... ({})",
                    style("quilt-server-launch.jar => ".to_owned() + &serverjar_name).dim()
                );

                fs::rename(
                    output_dir.join("quilt-server-launch.jar"),
                    output_dir.join(&serverjar_name),
                )
                .context("Renaming quilt-server-launch.jar")?;
            }

            serverjar_name
        }
        Downloadable::BuildTools { args } => {
            let installerjar_name = server.jar.get_filename(server, http_client).await?;
            if output_dir.join(installerjar_name.clone()).exists() {
                println!(
                    "          BuildTools present ({})",
                    style(installerjar_name.clone()).dim()
                );
            } else {
                println!(
                    "          Downloading BuildTools... ({})",
                    style(installerjar_name.clone()).dim()
                );

                let filename = &installerjar_name;
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
            }

            let serverjar_name = format!("spigot-{}.jar", server.mc_version);

            if output_dir.join(serverjar_name.clone()).exists() {
                println!(
                    "          Skipping server jar ({})",
                    style(serverjar_name.clone()).dim()
                );
            } else {
                println!("          Running BuildTools...",);

                let mut exec_args = vec![
                    "-jar",
                    &installerjar_name,
                    "--rev",
                    &server.mc_version,
                ];

                for arg in args {
                    exec_args.push(arg);
                }

                execute_child(
                    std::process::Command::new("java")
                        .args(exec_args)
                        .current_dir(output_dir),
                    output_dir,
                    "BuildTools",
                    "bt",
                )
                .await
                .context("Executing BuildTools")?;

                println!(
                    "          Renaming... ({})",
                    style("server.jar => ".to_owned() + &serverjar_name).dim()
                );

                fs::rename(
                    output_dir.join("server.jar"),
                    output_dir.join(&serverjar_name),
                )
                .context("Renaming server.jar")?;
            }

            serverjar_name
        }
        dl => {
            let serverjar_name = dl.get_filename(server, http_client).await?;
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

                let filename = &dl.get_filename(server, http_client).await?;
                util::download_with_progress(
                    File::create(&output_dir.join(filename))
                        .await
                        .context(format!("Failed to create output file for {filename}"))?,
                    filename,
                    dl,
                    server,
                    http_client,
                )
                .await?;
            }

            serverjar_name
        }
    };

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

fn create_scripts(server: &Server, serverjar_name: &str, output_dir: &Path) -> Result<()> {
    fs::write(
        output_dir.join("start.bat"),
        server
            .launcher
            .generate_script_win(serverjar_name, &server.name),
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
            .generate_script_linux(serverjar_name, &server.name)
            .as_bytes(),
    )?;

    println!(
        "          {}",
        style("start.bat and start.sh created").dim()
    );

    Ok(())
}
