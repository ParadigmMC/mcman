use anyhow::{anyhow, bail, Context, Result};
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use indexmap::IndexMap;
use pathdiff::diff_paths;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};
use tempfile::TempDir;
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipArchive};

use crate::{
    downloadable::{
        sources::{
            curserinth::{fetch_curserinth_project, fetch_curserinth_versions},
            modrinth::{
                fetch_modrinth_project, fetch_modrinth_versions, DependencyType, ModrinthVersion,
            },
        },
        Downloadable,
    },
    model::{ClientSideMod, Server},
    util::download_with_progress,
};

use super::md::MarkdownTable;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MRPackIndex {
    pub game: String,
    pub name: String,
    pub version_id: String,
    pub summary: Option<String>,
    pub files: Vec<MRPackFile>,
    pub dependencies: HashMap<String, String>,
}

impl MRPackIndex {
    pub async fn import_all(
        &self,
        server: &mut Server,
        http_client: &reqwest::Client,
    ) -> Result<()> {
        let len = self.files.len();
        let idx_w = len.to_string().len();
        for (idx, f) in self.files.iter().enumerate() {
            let url = f.downloads.first().context("unwrap url from downloads")?;

            let dl = Downloadable::from_url_interactive(http_client, server, url, false).await?;

            if f.env.is_none() || f.env.as_ref().unwrap().server != EnvSupport::Unsupported {
                server.mods.push(dl.clone());

                println!(
                    " > ({:idx_w$}/{len}) Imported {}",
                    idx + 1,
                    dl.to_short_string()
                );
            } else {
                // clientside only
                server.clientsidemods.push(ClientSideMod {
                    dl: dl.clone(),
                    optional: false,
                    desc: String::new(),
                });

                println!(
                    " > ({:idx_w$}/{len}) Imported {} as client side mod",
                    idx + 1,
                    dl.to_short_string()
                );
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MRPackFile {
    path: String,
    hashes: HashMap<String, String>,
    env: Option<Env>,
    downloads: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Env {
    pub client: EnvSupport,
    pub server: EnvSupport,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EnvSupport {
    Required,
    Optional,
    Unsupported,
}

pub async fn mrpack_source_to_file(
    src: &str,
    http_client: &reqwest::Client,
    tmp_dir: &TempDir,
    server: &Server,
) -> Result<File> {
    let filename = if src.starts_with("http") || src.starts_with("mr:") {
        let filename = tmp_dir.path().join("pack.mrpack");
        let file = tokio::fs::File::create(&filename).await?;

        let downloadable = resolve_mrpack_source(src, http_client).await?;

        println!(" > {}", style("Downloading mrpack...").green());

        download_with_progress(
            file,
            &format!("Downloading {src}..."),
            &downloadable,
            None,
            server,
            http_client,
        )
        .await?;

        filename
    } else {
        PathBuf::from(src)
    };

    File::open(filename).context("opening file")
}

#[allow(clippy::needless_pass_by_ref_mut)] // Yes it is used mutably, clippy.
pub async fn import_from_mrpack<R: Read + Seek>(
    server: &mut Server,
    http_client: &reqwest::Client,
    reader: R,
) -> Result<MRPackIndex> {
    println!(" > {}", style("Importing mrpack...").cyan());

    let mut archive = ZipArchive::new(reader).context("reading mrpack zip archive")?;

    println!(" > {}", style("Reading index...").cyan());

    let pack = mrpack_read_index(&mut archive)?;

    pack.import_all(server, http_client)
        .await
        .context("importing from mrpack index")?;

    println!(" > {}", style("Extracting overrides...").cyan().bold());

    let len = mrpack_import_configs(server, &mut archive)?;

    println!(
        " > {} {} {} {} {}",
        style("Imported").cyan().bold(),
        style(pack.files.len()).green(),
        style("mods and").cyan(),
        style(len).green(),
        style("config files").cyan(),
    );

    Ok(pack)
}

pub fn mrpack_read_index<R: Read + Seek>(archive: &mut ZipArchive<R>) -> Result<MRPackIndex> {
    let mut mr_index = archive.by_name("modrinth.index.json")?;
    let mut zip_content = Vec::new();
    mr_index
        .read_to_end(&mut zip_content)
        .context("reading modrinth.index.json from zip file")?;

    let pack: MRPackIndex = serde_json::from_slice(&zip_content)?;

    Ok(pack)
}

pub fn mrpack_import_configs<R: Read + Seek>(
    server: &Server,
    archive: &mut ZipArchive<R>,
) -> Result<usize> {
    let mut server_overrided = vec![];
    let mut queue = vec![];

    for filename in archive.file_names() {
        if filename.ends_with('/') {
            continue; // folder
        }

        let path = PathBuf::from(filename);

        let real_path = if path.starts_with("overrides") {
            if server_overrided.contains(&path) {
                continue;
            }

            server
                .path
                .join("config")
                .join(path.strip_prefix("overrides")?)
        } else if path.starts_with("server-overrides") {
            server_overrided.push(path.clone());
            server
                .path
                .join("config")
                .join(path.strip_prefix("server-overrides")?)
        } else {
            continue;
        };

        queue.push((path, real_path));
    }

    let len = queue.len();
    let idx_w = len.to_string().len();
    for (idx, (path, real_path)) in queue.iter().enumerate() {
        let mut zip_file = archive.by_name(&path.to_string_lossy())?;
        let mut zip_content = Vec::new();
        zip_file
            .read_to_end(&mut zip_content)
            .context(format!("reading {} from zip file", path.display()))?;

        fs::create_dir_all(real_path.parent().unwrap()).context(format!(
            "Creating parent folder for {}",
            real_path.display()
        ))?;

        fs::write(real_path, zip_content).context(format!("Writing {}", real_path.display()))?;

        println!(
            " > ({:idx_w$}/{len}) Config file: {}",
            idx + 1,
            style(path.to_string_lossy()).dim()
        );
    }

    Ok(len)
}

pub fn select_modrinth_version(
    list: &[ModrinthVersion],
    server: &Option<Server>,
) -> ModrinthVersion {
    let mut table = MarkdownTable::new();

    let list = list
        .iter()
        .filter(|v| {
            if let Some(serv) = &server {
                if !v.game_versions.contains(&serv.mc_version) {
                    return false;
                }
            }
            true
        })
        .collect::<Vec<_>>();

    for v in &list {
        let mut map = IndexMap::new();

        map.insert("num".to_owned(), v.version_number.clone());
        map.insert("name".to_owned(), v.name.clone());
        map.insert("compat".to_owned(), v.loaders.join(","));

        table.add_from_map(&map);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("  Which version?")
        .default(0)
        .items(&table.render_ascii_lines(false))
        .interact()
        .unwrap();

    list[selection].clone()
}

pub async fn resolve_mrpack_source(
    src: &str,
    http_client: &reqwest::Client,
) -> Result<Downloadable> {
    println!(" > {}", style("Resolving mrpack...").green());

    let modpack_id = if src.starts_with("mr:") {
        Some(src.strip_prefix("mr:").unwrap().to_owned())
    } else {
        let url = Url::parse(src)?;

        if url.domain() == Some("modrinth.com") && url.path().starts_with("/modpack") {
            url.path().strip_prefix("/modpack/").map(str::to_owned)
        } else {
            None
        }
    };

    let downloadable = if let Some(id) = modpack_id {
        let versions: Vec<ModrinthVersion> =
            fetch_modrinth_versions(http_client, &id, None).await?;

        let version = select_modrinth_version(&versions, &None);

        Downloadable::Modrinth {
            id: id.clone(),
            version: version.id,
        }
    } else {
        Downloadable::Url {
            url: src.to_owned(),
            filename: None,
            desc: None,
        }
    };

    Ok(downloadable)
}

#[allow(clippy::too_many_lines)]
pub async fn export_mrpack<W: std::io::Write + std::io::Seek>(
    http_client: &reqwest::Client,
    server: &Server,
    summary: Option<String>,
    version_id: &str,
    writer: W,
) -> Result<()> {
    let mut archive = zip::write::ZipWriter::new(writer);

    let mut dependencies = HashMap::new();

    dependencies.insert("minecraft".to_owned(), server.mc_version.clone());

    match &server.jar {
        Downloadable::Quilt { loader, .. } => {
            dependencies.insert("quilt-loader".to_owned(), loader.clone())
        }
        Downloadable::Fabric { loader, .. } => {
            dependencies.insert("fabric-loader".to_owned(), loader.clone())
        }
        _ => None,
    };

    let to_env = |dt: &DependencyType| match dt {
        DependencyType::Optional => EnvSupport::Optional,
        DependencyType::Unsupported | DependencyType::Incompatible => EnvSupport::Unsupported,
        _ => EnvSupport::Required,
    };

    let mut files = vec![];

    println!(" > {}", style("Converting mods...").cyan().bold());

    let len = server.mods.len();
    let idx_w = len.to_string().len();
    for (idx, serv_mod) in server.mods.iter().enumerate() {
        match serv_mod {
            Downloadable::Modrinth { id, version } => {
                let proj = fetch_modrinth_project(http_client, id).await?;

                let project = fetch_modrinth_versions(http_client, id, None).await?;

                let verdata = match version.as_str() {
                    "latest" => project.first(),
                    id => project.iter().find(|&v| v.id == id),
                }
                .ok_or(anyhow!(format!(
                    "Cant find modrinth version of {id}, ver={version}"
                )))?;

                // bad unwrap?
                let file = verdata.files.first().unwrap();

                files.push(MRPackFile {
                    hashes: file.hashes.clone(),
                    env: Some(Env {
                        client: to_env(&proj.client_side),
                        server: to_env(&proj.server_side),
                    }),
                    path: format!("mods/{}", file.filename),
                    downloads: vec![file.url.clone()],
                });

                println!(
                    " > ({:idx_w$}/{len}) {} {}",
                    idx + 1,
                    style("Converted").green(),
                    serv_mod.to_short_string()
                );
            }
            Downloadable::CurseRinth { id, version } => {
                let proj = fetch_curserinth_project(http_client, id).await?;

                let project = fetch_curserinth_versions(http_client, id, None).await?;

                let verdata = match version.as_str() {
                    "latest" => project.first(),
                    id => project.iter().find(|&v| v.id == id),
                }
                .ok_or(anyhow!(format!(
                    "Cant find curserinth version of {id}, ver={version}"
                )))?;

                // bad unwrap #2?
                let file = verdata.files.first().unwrap();

                files.push(MRPackFile {
                    hashes: file.hashes.clone(), // ! doesnt include sha512, thanks curseforge -_-
                    env: Some(Env {
                        client: to_env(&proj.client_side),
                        server: to_env(&proj.server_side),
                    }),
                    path: format!("mods/{}", file.filename),
                    downloads: vec![file.url.clone()],
                });

                println!(
                    " > ({:idx_w$}/{len}) {} {}",
                    idx + 1,
                    style("Converted").green(),
                    serv_mod.to_short_string()
                );
            }
            dl => {
                let filename = dl.get_filename(server, http_client).await?;
                if let Ok(url) = dl.get_url(http_client, Some(&filename)).await {
                    files.push(MRPackFile {
                        hashes: HashMap::new(), // ! todo...???
                        env: None,
                        path: format!("mods/{filename}"),
                        downloads: vec![url],
                    });

                    println!(
                        " > ({:idx_w$}/{len}) {} {}",
                        idx + 1,
                        style("Converted").green(),
                        serv_mod.to_short_string()
                    );
                } else {
                    println!(
                        " > ({:idx_w$}/{len}) {} {}",
                        idx + 1,
                        style("Skipped").yellow().bold(),
                        serv_mod.to_short_string()
                    );
                }
            }
        }
    }

    println!(
        " > {}",
        style("Converting client-side mods...").cyan().bold()
    );

    let len = server.clientsidemods.len();
    let idx_w = len.to_string().len();
    for (idx, client_mod) in server.clientsidemods.iter().enumerate() {
        match &client_mod.dl {
            Downloadable::Modrinth { id, version } => {
                let proj = fetch_modrinth_project(http_client, id).await?;

                let project = fetch_modrinth_versions(http_client, id, None).await?;

                let verdata = match version.as_str() {
                    "latest" => project.first(),
                    id => project.iter().find(|&v| v.id == id),
                }
                .ok_or(anyhow!(format!(
                    "Cant find modrinth version of {id}, ver={version}"
                )))?;

                // bad unwrap?
                let file = verdata.files.first().unwrap();

                files.push(MRPackFile {
                    hashes: file.hashes.clone(),
                    env: Some(Env {
                        client: if client_mod.optional {
                            EnvSupport::Optional
                        } else {
                            to_env(&proj.client_side)
                        },
                        server: to_env(&proj.server_side),
                    }),
                    path: format!("mods/{}", file.filename),
                    downloads: vec![file.url.clone()],
                });

                println!(
                    " > ({:idx_w$}/{len}) {} {}",
                    idx + 1,
                    style("Converted").green(),
                    client_mod.dl.to_short_string()
                );
            }
            Downloadable::CurseRinth { id, version } => {
                let proj = fetch_curserinth_project(http_client, id).await?;

                let project = fetch_curserinth_versions(http_client, id, None).await?;

                let verdata = match version.as_str() {
                    "latest" => project.first(),
                    id => project.iter().find(|&v| v.id == id),
                }
                .ok_or(anyhow!(format!(
                    "Cant find curserinth version of {id}, ver={version}"
                )))?;

                // bad unwrap #2?
                let file = verdata.files.first().unwrap();

                files.push(MRPackFile {
                    hashes: file.hashes.clone(), // ! doesnt include sha512, thanks curseforge -_-
                    env: Some(Env {
                        client: if client_mod.optional {
                            EnvSupport::Optional
                        } else {
                            to_env(&proj.client_side)
                        },
                        server: to_env(&proj.server_side),
                    }),
                    path: format!("mods/{}", file.filename),
                    downloads: vec![file.url.clone()],
                });

                println!(
                    " > ({:idx_w$}/{len}) {} {}",
                    idx + 1,
                    style("Converted").green(),
                    client_mod.dl.to_short_string()
                );
            }
            dl => {
                let filename = dl.get_filename(server, http_client).await?;
                if let Ok(url) = dl.get_url(http_client, Some(&filename)).await {
                    files.push(MRPackFile {
                        hashes: HashMap::new(), // ! todo...???
                        env: None,
                        path: format!("mods/{filename}"),
                        downloads: vec![url],
                    });

                    println!(
                        " > ({:idx_w$}/{len}) {} {}",
                        idx + 1,
                        style("Converted").green(),
                        client_mod.dl.to_short_string()
                    );
                } else {
                    println!(
                        " > ({:idx_w$}/{len}) {} {}",
                        idx + 1,
                        style("Skipped").yellow().bold(),
                        client_mod.dl.to_short_string()
                    );
                }
            }
        }
    }

    let index = MRPackIndex {
        name: server
            .variables
            .get("MODPACK_NAME")
            .cloned()
            .unwrap_or(server.name.clone()),
        summary: if summary.is_some() {
            summary
        } else {
            server.variables.get("MODPACK_SUMMARY").cloned()
        },
        dependencies,
        game: "minecraft".to_owned(),
        files,
        version_id: version_id.to_owned(),
    };

    println!(" > {}", style("Writing index...").cyan().bold());

    archive.start_file("modrinth.index.json", FileOptions::default())?;

    archive.write_all(serde_json::to_string_pretty(&index)?.as_bytes())?;

    if server.path.join("config").exists() {
        println!(" > {}", style("Writing config/...").cyan().bold());

        for entry in WalkDir::new(server.path.join("config")) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    bail!(
                        "Can't walk directory/file {}: {e}",
                        &e.path().unwrap_or(Path::new("unknown")).display()
                    );
                }
            };

            let rel_path = diff_paths(entry.path(), &server.path.join("config"))
                .ok_or(anyhow!("Cannot diff paths"))?;

            let dest_path = "overrides/".to_owned() + &rel_path.to_string_lossy();

            if entry.file_type().is_dir() {
                archive
                    .add_directory(dest_path.clone(), FileOptions::default())
                    .context(format!("Creating dir in zip: {dest_path}"))?;
            } else {
                archive
                    .start_file(dest_path.clone(), FileOptions::default())
                    .context(format!("Starting zip file for {dest_path}"))?;

                let mut real_file =
                    fs::File::open(entry.path()).context(format!("Opening file {dest_path}"))?;

                std::io::copy(&mut real_file, &mut archive)
                    .context(format!("Copying {dest_path} to in-memory zip archive"))?;

                println!("    -> {}", style(dest_path).dim());
            }
        }
    }

    if server.path.join("client-config").exists() {
        println!(" > {}", style("Writing client-config/...").cyan().bold());

        for entry in WalkDir::new(server.path.join("client-config")) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    bail!(
                        "Can't walk directory/file {}: {e}",
                        &e.path().unwrap_or(Path::new("unknown")).display()
                    );
                }
            };

            let rel_path = diff_paths(entry.path(), &server.path.join("client-config"))
                .ok_or(anyhow!("Cannot diff paths"))?;

            let dest_path = "client-overrides/".to_owned() + &rel_path.to_string_lossy();

            if entry.file_type().is_dir() {
                archive
                    .add_directory(dest_path.clone(), FileOptions::default())
                    .context(format!("Creating dir in zip: {dest_path}"))?;
            } else {
                archive
                    .start_file(dest_path.clone(), FileOptions::default())
                    .context(format!("Starting zip file for {dest_path}"))?;

                let mut real_file =
                    fs::File::open(entry.path()).context(format!("Opening file {dest_path}"))?;

                std::io::copy(&mut real_file, &mut archive)
                    .context(format!("Copying {dest_path} to in-memory zip archive"))?;

                println!("    -> {}", style(dest_path).dim());
            }
        }
    }

    archive.finish().context("Finishing zip archive")?;

    println!(" > {}", style("Export complete!").green().bold());

    Ok(())
}
