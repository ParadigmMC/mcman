use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context, Result};
use console::style;
use futures::StreamExt;
use pathdiff::diff_paths;
use reqwest::{IntoUrl, Url};
use rpackwiz::model::{
    CurseforgeModUpdate, DownloadMode, HashFormat, Mod, ModDownload, ModOption, ModUpdate,
    ModrinthModUpdate, Pack, PackFile, PackIndex, Side,
};
use sha2::{Digest, Sha256};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use walkdir::WalkDir;

use crate::{
    downloadable::{
        sources::{
            curserinth::{fetch_curserinth_project, fetch_curserinth_versions},
            modrinth::{fetch_modrinth_project, fetch_modrinth_versions, DependencyType},
        },
        Downloadable,
    },
    model::{ClientSideMod, Server},
    util::download_with_progress,
};

pub struct PackwizExportOptions {
    /// false -> use metadata:curseforge, true -> use edge.forgecdn.net
    pub cf_usecdn: bool,
}

pub async fn packwiz_import_from_source(
    http_client: &reqwest::Client,
    src: &str,
    server: &mut Server,
) -> Result<(Pack, usize, usize)> {
    Ok(if src.starts_with("http") {
        let base_url = Url::parse(src).context("Parsing source url")?;

        packwiz_import_http(http_client, base_url, server).await?
    } else {
        let base = PathBuf::from(src);

        packwiz_import_local(http_client, base, server).await?
    })
}

// bad code #99999
pub async fn packwiz_fetch_pack_from_src(http_client: &reqwest::Client, src: &str) -> Result<Pack> {
    Ok(if src.starts_with("http") {
        let base_url = Url::parse(src).context("Parsing source url")?;

        fetch_toml(http_client, base_url.clone())
            .await
            .context("Fetching pack.toml")?
    } else {
        let base = PathBuf::from(src);

        let base = if base.ends_with("pack.toml") {
            base
        } else {
            base.join("pack.toml")
        };

        read_toml(&base).await.context("Reading pack.toml")?
    })
}

pub async fn packwiz_import_http(
    http_client: &reqwest::Client,
    base_url: reqwest::Url,
    server: &mut Server,
) -> Result<(Pack, usize, usize)> {
    let pack: Pack = fetch_toml(http_client, base_url.clone())
        .await
        .context("Fetching pack.toml")?;

    let index_url = base_url
        .join(&pack.index.file)
        .context("Resolving pack index url")?;

    println!(" > {}", style("Fetching index...").dim());

    let pack_index: PackIndex = fetch_toml(http_client, index_url)
        .await
        .context("Fetching pack index")?;

    let mut mod_count = 0;
    let mut config_count = 0;

    let idx_len = pack_index.files.len();
    let idx_w = idx_len.to_string().len();
    for (idx, file) in pack_index.files.iter().enumerate() {
        let file_url = base_url
            .join(&file.file)
            .context("Resolving pack file url")?;
        if file.metafile {
            println!(
                " > ({:idx_w$}/{idx_len}) {} {}",
                idx + 1,
                style("Importing metafile:").green(),
                file.file
            );

            let m: Mod = fetch_toml(http_client, file_url)
                .await
                .context("Fetching metafile toml")?;

            let Some(dl) = pw_mod_to_dl(&m, http_client, server).await? else {
                continue;
            };

            if m.side == Side::Client {
                println!(
                    "   {:w$} {} {} {}",
                    "",
                    style("-> Imported from").dim(),
                    dl.to_short_string(),
                    style("as clientside").bold(),
                    w = (idx_w * 2) + 3,
                );

                server.clientsidemods.push(ClientSideMod {
                    dl,
                    desc: m.option.description.unwrap_or_default(),
                    optional: m.option.optional,
                });
            } else {
                println!(
                    "   {:w$} {} {}",
                    "",
                    style("-> Imported from").dim(),
                    dl.to_short_string(),
                    w = (idx_w * 2) + 3,
                );

                server.mods.push(dl);
            }

            mod_count += 1;
        } else {
            println!(
                " > ({:idx_w$}/{idx_len}) {} {}",
                idx + 1,
                style("Config file:").green(),
                file.file
            );

            let dest_path = server.path.join("config").join(&file.file);

            fs::create_dir_all(dest_path.parent().expect("Parent to be Some"))
                .await
                .context(format!(
                    "Creating parent dir for {}",
                    dest_path.to_string_lossy()
                ))?;

            download_with_progress(
                File::create(&dest_path)
                    .await
                    .context(format!("Creating file {}", dest_path.to_string_lossy()))?,
                &file.file,
                &Downloadable::Url {
                    url: file_url.as_str().to_owned(),
                    filename: None,
                    desc: None,
                },
                None, //unneeded
                server,
                http_client,
            )
            .await
            .context(format!("Downloading {} from {file_url}", file.file))?;

            config_count += 1;
        }
    }

    Ok((pack, mod_count, config_count))
}

pub async fn pw_mod_to_dl(
    m: &Mod,
    http_client: &reqwest::Client,
    server: &Server,
) -> Result<Option<Downloadable>> {
    Ok(Some(if let Some(upd) = &m.update {
        if let Some(mr) = &upd.modrinth {
            Downloadable::Modrinth {
                id: mr.mod_id.clone(),
                version: mr.version.clone(),
            }
        } else if let Some(cf) = &upd.curseforge {
            Downloadable::CurseRinth {
                id: cf.project_id.to_string(),
                version: cf.file_id.to_string(),
            }
        } else {
            println!("ERROR: UNKNOWN MOD UPDATE");
            return Ok(None); // Hell
        }
    } else {
        Downloadable::from_url_interactive(
            http_client,
            server,
            &m.download
                .url
                .clone()
                .ok_or(anyhow!("download url not present"))?,
            false,
        )
        .await
        .context("Resolving Downloadable from URL")?
    }))
}

pub async fn packwiz_import_local(
    http_client: &reqwest::Client,
    base: PathBuf,
    server: &mut Server,
) -> Result<(Pack, usize, usize)> {
    let base = if base.ends_with("pack.toml") {
        base
    } else {
        base.join("pack.toml")
    };

    let pack: Pack = read_toml(&base).await.context("Reading pack.toml")?;

    println!(" > {}", style("Reading index...").dim());

    let pack_index: PackIndex = read_toml(&base.join(&pack.index.file))
        .await
        .context("Reading pack index file")?;

    let mut mod_count = 0;
    let mut config_count = 0;

    let idx_len = pack_index.files.len();
    let idx_w = idx_len.to_string().len();
    for (idx, file) in pack_index.files.iter().enumerate() {
        let file_path = base.join(&file.file);
        if file.metafile {
            println!(
                " > ({:idx_w$}/{idx_len}) {} {}",
                idx + 1,
                style("Importing metafile:").green(),
                file.file
            );

            let m: Mod = read_toml(&file_path)
                .await
                .context(format!("Reading toml from {}", file_path.to_string_lossy()))?;

            let Some(dl) = pw_mod_to_dl(&m, http_client, server).await? else {
                continue;
            };

            if m.side == Side::Client {
                println!(
                    "   {:w$} {} {} {}",
                    "",
                    style("-> Imported from").dim(),
                    dl.to_short_string(),
                    style("as clientside").bold(),
                    w = (idx_w * 2) + 3,
                );

                server.clientsidemods.push(ClientSideMod {
                    dl,
                    desc: m.option.description.unwrap_or_default(),
                    optional: m.option.optional,
                });
            } else {
                println!(
                    "   {:w$} {} {}",
                    "",
                    style("-> Imported from").dim(),
                    dl.to_short_string(),
                    w = (idx_w * 2) + 3,
                );

                server.mods.push(dl);
            }

            mod_count += 1;
        } else {
            println!(
                " > ({:idx_w$}/{idx_len}) {} {}",
                idx + 1,
                style("Config file:").green(),
                file.file
            );

            let dest_path = server.path.join("config").join(&file.file);

            fs::create_dir_all(dest_path.parent().expect("Parent to be Some"))
                .await
                .context(format!(
                    "Creating parent dir for {}",
                    dest_path.to_string_lossy()
                ))?;

            fs::copy(&file.file, dest_path).await?;

            config_count += 1;
        }
    }

    Ok((pack, mod_count, config_count))
}

pub async fn fetch_toml<T, U>(http_client: &reqwest::Client, url: U) -> Result<T>
where
    T: serde::de::DeserializeOwned,
    U: IntoUrl,
{
    let contents = http_client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(toml::from_str(&contents)?)
}

pub async fn read_toml<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T> {
    let str = fs::read_to_string(path).await?;
    Ok(toml::from_str(&str)?)
}

#[allow(clippy::too_many_lines)]
pub async fn export_packwiz(
    folder: &PathBuf,
    http_client: &reqwest::Client,
    server: &Server,
    opts: &PackwizExportOptions,
) -> Result<()> {
    fs::create_dir_all(folder)
        .await
        .context("creating output folder")?;

    let mut pack_index = PackIndex {
        files: vec![],
        hash_format: HashFormat::Sha256,
    };

    println!(" > {}", style("Creating mod metafiles...").cyan());

    let metafiles = create_packwiz_modlist(http_client, server, opts)
        .await
        .context("Creating packwiz mod metafile list")?;

    println!(" > {}", style("Writing mod metafiles...").cyan());

    let len = metafiles.len();
    let idx_w = len.to_string().len();
    for (idx, (name, metafile)) in metafiles.iter().enumerate() {
        let rel_path = "mods/".to_owned() + name;
        let path = folder.join(&rel_path);
        let contents = toml::to_string_pretty(metafile).context("serializing pw mod")?;
        fs::create_dir_all(path.parent().expect("parent of dest present")).await?;
        fs::write(&path, &contents)
            .await
            .context(format!("Writing {name} metafile"))?;

        pack_index.files.push(PackFile {
            file: rel_path.clone(),
            hash: hash_contents(&contents),
            metafile: true,

            hash_format: None,
            alias: None,
            preserve: false, // ?
        });

        println!(
            "   ({:idx_w$}/{len}) {} {rel_path}",
            idx + 1,
            style("Mod:").green()
        );
    }

    if server.path.join("client-config").exists() {
        println!(" > {}", style("Writing client-config/...").cyan());

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

            let dest_path = folder.join(&rel_path);

            if entry.file_type().is_dir() {
                continue;
            }

            fs::create_dir_all(dest_path.parent().expect("parent of dest present")).await?;

            // TODO: bootstrapping
            fs::copy(entry.path(), &dest_path).await.context(format!(
                "Copying {} to {}",
                entry.path().to_string_lossy(),
                dest_path.to_string_lossy()
            ))?;

            pack_index.files.push(PackFile {
                file: rel_path.to_string_lossy().into_owned(), // maybe problematic?
                hash: hash_file(&dest_path)?,
                metafile: true,

                hash_format: None,
                alias: None,
                preserve: false, // ?
            });

            println!("    -> {}", style(rel_path.to_string_lossy()).dim());
        }
    }

    println!(" > {}", style("Writing pack and index...").cyan());

    let mut f = File::create(folder.join("index.toml")).await?;
    f.write_all(toml::to_string_pretty(&pack_index)?.as_bytes())
        .await?;

    let mut versions = HashMap::new();

    versions.insert("minecraft".to_owned(), server.mc_version.clone());

    match &server.jar {
        Downloadable::Quilt { loader, .. } => versions.insert("quilt".to_owned(), loader.clone()),
        Downloadable::Fabric { loader, .. } => versions.insert("fabric".to_owned(), loader.clone()),
        _ => None,
    };

    let pack = Pack {
        index: PackFile {
            file: "index.toml".to_owned(),
            hash_format: Some("sha256".to_owned()),
            hash: hash_file(&folder.join("index.toml"))?,
            alias: None,
            metafile: false,
            preserve: false,
        },
        pack_format: "packwiz:1.1.0".to_owned(),
        name: if let Some(n) = server.variables.get("MODPACK_NAME") {
            n.clone()
        } else {
            server.name.clone()
        },
        author: server.variables.get("MODPACK_AUTHORS").cloned(),
        description: server.variables.get("MODPACK_SUMMARY").cloned(),
        version: server.variables.get("MODPACK_VERSION").cloned(),
        versions,
    };

    let mut f = File::create(folder.join("pack.toml")).await?;
    f.write_all(toml::to_string_pretty(&pack)?.as_bytes())
        .await?;

    println!(
        " > {}",
        style("Exported to packwiz successfully!").green().bold()
    );

    if let Ok(u) = try_get_url(folder) {
        println!();
        println!(" > {}", style("Exported pack URL:").cyan());
        println!("     {}", "https://raw.githack.com/".to_owned() + &u);
        println!(" > {}", style("MultiMC prelaunch command:").cyan());
        println!(
            "     {}",
            "$INST_JAVA -jar packwiz-installer-bootstrap.jar https://raw.githack.com/".to_owned()
                + &u
        );
        println!();
    }

    Ok(())
}

pub fn try_get_url(folder: &PathBuf) -> Result<String> {
    let repo_url = get_git_remote()?.ok_or(anyhow!("cant get repo url"))?;
    let root = get_git_root()?.ok_or(anyhow!("cant get repo root"))?;
    let branch = get_git_branch()?.ok_or(anyhow!("cant get repo branch"))?;

    let diff = diff_paths(folder, root).ok_or(anyhow!("cant diff paths"))?;

    let repo = if repo_url.starts_with("https") {
        repo_url.strip_prefix("https://github.com/")
    } else {
        repo_url.strip_prefix("http://github.com/")
    }
    .ok_or(anyhow!("repo not on github?"))?;

    let parent_paths = diff.to_string_lossy().replace('\\', "/");
    let parent_paths = if parent_paths.is_empty() {
        parent_paths
    } else {
        "/".to_owned() + &parent_paths
    };

    Ok(repo.to_owned() + "/" + &branch + &parent_paths + "/pack.toml")
}

pub fn get_git_remote() -> Result<Option<String>> {
    let path = git_command(vec!["remote", "get-url", "origin"])?
        .ok_or(anyhow!("cant get git repo origin"))?;

    Ok(Some(
        path.strip_suffix(".git")
            .map_or(path.clone(), ToOwned::to_owned),
    ))
}

pub fn get_git_root() -> Result<Option<String>> {
    git_command(vec!["rev-parse", "--show-toplevel"])
}

pub fn get_git_branch() -> Result<Option<String>> {
    git_command(vec!["rev-parse", "--abbrev-ref", "HEAD"])
}

pub fn git_command(args: Vec<&str>) -> Result<Option<String>> {
    let output = std::process::Command::new("git").args(args).output()?;

    Ok(if output.status.success() {
        let path = String::from_utf8_lossy(output.stdout.as_slice())
            .into_owned()
            .trim()
            .to_owned();
        Some(path)
    } else {
        None
    })
}

pub async fn create_packwiz_modlist(
    http_client: &reqwest::Client,
    server: &Server,
    opts: &PackwizExportOptions,
) -> Result<Vec<(String, Mod)>> {
    let mut list = vec![];

    for dl in &server.mods {
        if let Some(t) = dl_to_pw_mod(dl, http_client, server, opts, None, "").await? {
            list.push(t);
        }
    }

    for client_mod in &server.clientsidemods {
        if let Some(t) = dl_to_pw_mod(
            &client_mod.dl,
            http_client,
            server,
            opts,
            Some(client_mod.optional),
            &client_mod.desc,
        )
        .await?
        {
            list.push(t);
        }
    }

    Ok(list)
}

#[allow(clippy::too_many_lines)] // xd
pub async fn dl_to_pw_mod(
    dl: &Downloadable,
    http_client: &reqwest::Client,
    server: &Server,
    opts: &PackwizExportOptions,
    is_opt: Option<bool>,
    desc_override: &str,
) -> Result<Option<(String, Mod)>> {
    Ok(match dl {
        Downloadable::Modrinth { id, version } => {
            let proj = fetch_modrinth_project(http_client, id).await?;

            let side = match (proj.server_side.clone(), proj.client_side.clone()) {
                (DependencyType::Incompatible | DependencyType::Unsupported, _) => Side::Client,
                (_, DependencyType::Incompatible | DependencyType::Unsupported) => Side::Server,
                _ => Side::Both,
            };

            let versions = fetch_modrinth_versions(http_client, id, None).await?;

            let verdata = if version == "latest" {
                versions.first()
            } else {
                versions.iter().find(|&v| v.id == version.clone())
            };

            let Some(verdata) = verdata else {
                bail!("Release '{version}' for project '{id}' not found");
            };

            let Some(file) = verdata.files.first() else {
                bail!("No files for project '{id}' version '{version}'");
            };

            let hash = file
                .hashes
                .get("sha512")
                .expect("modrinth to provide sha512 hashes")
                .clone();

            let m = Mod {
                name: proj.title,
                side,
                filename: file.filename.clone(),
                download: ModDownload {
                    mode: DownloadMode::Url,
                    url: Some(file.url.clone()),
                    hash,
                    hash_format: HashFormat::Sha512,
                },
                option: ModOption {
                    optional: is_opt.unwrap_or(proj.client_side == DependencyType::Optional),
                    default: false,
                    description: Some(if desc_override.is_empty() {
                        proj.description
                    } else {
                        desc_override.to_owned()
                    }),
                },
                update: Some(ModUpdate {
                    modrinth: Some(ModrinthModUpdate {
                        mod_id: proj.id,
                        version: version.clone(),
                    }),
                    curseforge: None,
                }),
            };

            Some((proj.slug + ".pw.toml", m))
        }

        Downloadable::CurseRinth { id, version } => {
            let proj = fetch_curserinth_project(http_client, id).await?;

            let side = match (proj.server_side.clone(), proj.client_side.clone()) {
                (DependencyType::Incompatible | DependencyType::Unsupported, _) => Side::Client,
                (_, DependencyType::Incompatible | DependencyType::Unsupported) => Side::Server,
                _ => Side::Both,
            };

            let versions = fetch_curserinth_versions(http_client, id, None).await?;

            let verdata = if version == "latest" {
                versions.first()
            } else {
                versions.iter().find(|&v| v.id == version.clone())
            };

            let Some(verdata) = verdata else {
                bail!("Release '{version}' for project '{id}' not found");
            };

            let Some(file) = verdata.files.first() else {
                bail!("No files for project '{id}' version '{version}'");
            };

            let hash = file
                .hashes
                .get("sha1")
                .expect("curserinth to provide sha1 hashes from cf")
                .clone();

            let m = Mod {
                name: proj.title,
                side,
                filename: file.filename.clone(),
                download: if opts.cf_usecdn {
                    ModDownload {
                        mode: DownloadMode::Url,
                        url: Some(file.url.clone()),
                        hash,
                        hash_format: HashFormat::Sha1,
                    }
                } else {
                    ModDownload {
                        mode: DownloadMode::Curseforge,
                        url: None,
                        hash,
                        hash_format: HashFormat::Sha1,
                    }
                },
                option: ModOption {
                    optional: is_opt.unwrap_or(proj.client_side == DependencyType::Optional),
                    default: false,
                    description: Some(if desc_override.is_empty() {
                        proj.description
                    } else {
                        desc_override.to_owned()
                    }),
                },
                update: if opts.cf_usecdn {
                    None
                } else {
                    Some(ModUpdate {
                        modrinth: None,
                        curseforge: Some(CurseforgeModUpdate {
                            file_id: verdata.id.parse()?,
                            project_id: verdata.project_id.parse()?,
                        }),
                    })
                },
            };

            Some((proj.slug + ".pw.toml", m))
        }

        Downloadable::Url { url, desc, .. } => {
            let filename = dl.get_filename(server, http_client).await?;

            let hash = get_hash_url(http_client, url).await?;

            let m = Mod {
                name: filename.clone(),
                side: Side::Both,
                filename: filename.clone(),
                download: ModDownload {
                    mode: DownloadMode::Url,
                    url: Some(url.clone()),
                    hash,
                    hash_format: HashFormat::Sha256,
                },
                option: ModOption {
                    optional: is_opt.unwrap_or(false),
                    default: false,
                    description: if desc_override.is_empty() {
                        desc.clone()
                    } else {
                        Some(desc_override.to_owned())
                    },
                },
                update: None,
            };

            Some((filename + ".pw.toml", m))
        }

        dl => {
            println!("WARNING: Cant make this a mod: {dl:#?}");
            None
        }
    })
}

pub fn hash_contents(contents: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(contents);

    // unholy hell
    let hash = (hasher.finalize().as_slice() as &[u8])
        .iter()
        .map(|b| format!("{b:x?}"))
        .collect::<String>();

    hash
}

pub fn hash_file(path: &PathBuf) -> Result<String> {
    let mut hasher = Sha256::new();

    let mut file = std::fs::File::open(path)?;

    std::io::copy(&mut file, &mut hasher)?;

    // unholy hell
    let hash = (hasher.finalize().as_slice() as &[u8])
        .iter()
        .map(|b| format!("{b:x?}"))
        .collect::<String>();

    Ok(hash)
}

pub async fn get_hash_url(client: &reqwest::Client, url: &str) -> Result<String> {
    // rust-analyzer broke
    let mut hasher = Sha256::new();

    let mut stream = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .bytes_stream();

    while let Some(item) = stream.next().await {
        let item = item?;
        hasher.update(item);
    }

    // unholy hell
    let hash = (hasher.finalize().as_slice() as &[u8])
        .iter()
        .map(|b| format!("{b:x?}"))
        .collect::<String>();

    Ok(hash)
}
