use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context, Result};
use console::style;
use pathdiff::diff_paths;
use reqwest::{IntoUrl, Url};
use rpackwiz::model::{
    HashFormat, Mod, Pack, PackFile, PackIndex, Side,
};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use walkdir::WalkDir;

use crate::{
    downloadable::Downloadable,
    model::{ClientSideMod, Server},
    util::{download_with_progress, hash::{hash_contents, hash_file}},
    util::env::try_get_url,
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
    Ok(if src.starts_with("http://") || src.starts_with("https://") {
        let base_url = Url::parse(src).context("Parsing source url")?;

        packwiz_import_http(http_client, base_url, server).await?
    } else {
        let base = PathBuf::from(src);

        packwiz_import_local(http_client, base, server).await?
    })
}

// bad code #99999
pub async fn packwiz_fetch_pack_from_src(http_client: &reqwest::Client, src: &str) -> Result<Pack> {
    Ok(if src.starts_with("http://") || src.starts_with("https://") {
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

#[allow(clippy::too_many_lines)]
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
            if file.file.starts_with("mods") {
                println!(
                    " > ({:idx_w$}/{idx_len}) {} {}",
                    idx + 1,
                    style("Importing mod:").green(),
                    file.file
                );
            } else {
                println!(
                    " > ({:idx_w$}/{idx_len}) {} {} {}",
                    idx + 1,
                    style("Skipping:").yellow().bold(),
                    file.file,
                    style("(unsupported)").yellow().bold(),
                );
                continue;
            }

            let m: Mod = fetch_toml(http_client, file_url)
                .await
                .context("Fetching metafile toml")?;

            let Some(dl) = Downloadable::from_pw_mod(&m, http_client, server).await? else {
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

pub async fn packwiz_import_local(
    http_client: &reqwest::Client,
    base: PathBuf,
    server: &mut Server,
) -> Result<(Pack, usize, usize)> {
    let base = if base.ends_with("pack.toml") {
        base.parent().ok_or(anyhow!("no parent of directory"))?.to_path_buf()
    } else {
        base
    };

    let pack: Pack = read_toml(&base.join("pack.toml")).await.context("Reading pack.toml")?;

    println!(" > {}", style("Reading index...").dim());

    let pack_index_path = base.join(&pack.index.file);

    let pack_index: PackIndex = read_toml(&pack_index_path)
        .await
        .context(format!("Reading pack index file {}", pack_index_path.to_string_lossy()))?;

    let mut mod_count = 0;
    let mut config_count = 0;

    let idx_len = pack_index.files.len();
    let idx_w = idx_len.to_string().len();
    for (idx, file) in pack_index.files.iter().enumerate() {
        let file_path = base.join(&file.file);
        if file.metafile {
            if file.file.starts_with("mods") {
                println!(
                    " > ({:idx_w$}/{idx_len}) {} {}",
                    idx + 1,
                    style("Importing mod:").green(),
                    file.file
                );
            } else {
                println!(
                    " > ({:idx_w$}/{idx_len}) {} {} {}",
                    idx + 1,
                    style("Skipping:").yellow().bold(),
                    file.file,
                    style("(unsupported)").yellow().bold(),
                );
                continue;
            }

            let m: Mod = read_toml(&file_path)
                .await
                .context(format!("Reading toml from {}", file_path.to_string_lossy()))?;

            let Some(dl) = Downloadable::from_pw_mod(&m, http_client, server).await? else {
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

    if let Ok(u) = try_get_url(&folder.join("pack.toml")) {
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

pub async fn create_packwiz_modlist(
    http_client: &reqwest::Client,
    server: &Server,
    opts: &PackwizExportOptions,
) -> Result<Vec<(String, Mod)>> {
    let mut list = vec![];

    for dl in &server.mods {
        if let Some(t) = dl.to_pw_mod(http_client, server, opts, None, "").await? {
            list.push(t);
        }
    }

    for client_mod in &server.clientsidemods {
        if let Some(t) = client_mod.dl.to_pw_mod(
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
