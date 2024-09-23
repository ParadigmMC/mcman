use std::{path::PathBuf, sync::Arc};

use anyhow::{Context, Result};

use crate::api::{
    app::App,
    models::{
        legacy::{LegacyDownloadable, LegacyServer, LegacyServerType},
        markdown::MarkdownOptions,
        server::{PaperMCProject, Server, ServerFlavor, ServerJar, ServerType},
        Addon, AddonListFile, AddonTarget, AddonType, Environment, Source, SourceType,
    },
    utils::toml::{read_toml, write_toml},
};

#[derive(clap::Args)]
pub struct Args {}

pub async fn run(_app: Arc<App>, _args: Args) -> Result<()> {
    if PathBuf::from("./server.toml").exists() {
        println!("Migrating server...");
        migrate_server().await?;
    } else if PathBuf::from("./network.toml").exists() {
        todo!();
    } else {
        println!("No server.toml or network.toml found in current directory to migrate");
    }

    Ok(())
}

pub fn migrate_downloadable(downloadable: LegacyDownloadable) -> AddonType {
    match downloadable {
        LegacyDownloadable::Url { url, .. } => AddonType::Url { url },
        LegacyDownloadable::Modrinth { id, version } => AddonType::Modrinth { id, version },
        LegacyDownloadable::CurseRinth { id, version } => AddonType::Curseforge { id, version },
        LegacyDownloadable::Spigot { id, version } => AddonType::Spigot { id, version },
        LegacyDownloadable::Hangar { id, version } => AddonType::Hangar { id, version },
        LegacyDownloadable::GithubRelease { repo, tag, asset } => AddonType::GithubRelease {
            repo,
            version: tag,
            filename: asset,
        },
        LegacyDownloadable::Jenkins {
            url,
            job,
            build,
            artifact,
        } => AddonType::Jenkins {
            url,
            job,
            build,
            artifact,
        },
        LegacyDownloadable::Maven {
            url,
            group,
            artifact,
            version,
            filename,
        } => AddonType::MavenArtifact {
            url,
            group,
            artifact,
            version,
            filename,
        },
    }
}

pub async fn migrate_server() -> Result<()> {
    let legacy_server = read_toml::<LegacyServer>(&PathBuf::from("./server.toml"))
        .with_context(|| "Reading server.toml")?;

    let mut addons: Vec<Addon> = vec![];

    for plugin in legacy_server.plugins {
        addons.push(Addon {
            addon_type: migrate_downloadable(plugin),
            environment: None,
            target: AddonTarget::Plugins,
        });
    }
    for m in legacy_server.mods {
        addons.push(Addon {
            addon_type: migrate_downloadable(m),
            environment: None,
            target: AddonTarget::Mods,
        });
    }
    for cmod in legacy_server.clientsidemods {
        addons.push(Addon {
            addon_type: migrate_downloadable(cmod.dl),
            environment: Some(Environment::Client),
            target: AddonTarget::Mods,
        });
    }

    let file = AddonListFile {
        addons,
        ..Default::default()
    };

    write_toml(&std::env::current_dir()?, "addons.toml", &file)
        .with_context(|| format!("Writing addons.toml"))?;

    let source = Source {
        source_type: SourceType::File {
            path: String::from("./addons.toml"),
        },
    };

    let server = Server {
        name: legacy_server.name,
        port: None,
        version: legacy_server.variables.get("MODPACK_VERSION").cloned(),
        launcher: legacy_server.launcher,
        markdown: MarkdownOptions {
            files: legacy_server.markdown.files,
            ..Default::default()
        },
        variables: legacy_server.variables,
        sources: vec![source],
        jar: Some(ServerJar {
            mc_version: legacy_server.mc_version,
            server_type: match legacy_server.jar {
                LegacyServerType::Vanilla {} => ServerType::Vanilla {},
                LegacyServerType::PaperMC { project, build } => ServerType::PaperMC {
                    project: serde_json::from_value::<PaperMCProject>(serde_json::Value::String(
                        project,
                    ))?,
                    build,
                },
                LegacyServerType::Purpur { build } => ServerType::Purpur { build },
                LegacyServerType::Fabric { loader, installer } => {
                    ServerType::Fabric { loader, installer }
                },
                LegacyServerType::Quilt { loader, installer } => {
                    ServerType::Quilt { loader, installer }
                },
                LegacyServerType::NeoForge { loader } => ServerType::NeoForge { loader },
                LegacyServerType::Forge { loader } => ServerType::Forge { loader },
                LegacyServerType::BuildTools { software, args } => ServerType::BuildTools {
                    craftbukkit: software == "craftbukkit",
                    args,
                },
                LegacyServerType::Paper {} => ServerType::PaperMC {
                    project: PaperMCProject::Paper,
                    build: String::from("latest"),
                },
                LegacyServerType::Velocity {} => ServerType::PaperMC {
                    project: PaperMCProject::Velocity,
                    build: String::from("latest"),
                },
                LegacyServerType::Waterfall {} => ServerType::PaperMC {
                    project: PaperMCProject::Waterfall,
                    build: String::from("latest"),
                },
                LegacyServerType::BungeeCord {} => todo!(),
                LegacyServerType::Downloadable { inner } => {
                    let flavor = ServerFlavor::Patched;

                    ServerType::Custom {
                        inner: migrate_downloadable(inner),
                        flavor,
                        exec: None,
                    }
                },
            },
        }),
    };

    std::fs::copy("./server.toml", "./__v1__server.toml")?;

    write_toml(&std::env::current_dir()?, "server.toml", &server)
        .with_context(|| format!("Writing server.toml"))?;

    println!("Migrated! You may now delete the backup file if there are no issues.");

    Ok(())
}
