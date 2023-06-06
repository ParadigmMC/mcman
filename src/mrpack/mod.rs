use std::{fs::{File, self}, io::{Read, self}, path::{Path, PathBuf}, collections::HashMap};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use tempfile::TempDir;
use zip::ZipArchive;
use console::style;

use crate::{model::Server, bootstrapper::{bootstrap, BootstrapContext}};

#[derive(Debug, Deserialize, Serialize)]
#[serde()]
pub struct MRPackIndex {
    pub name: String,
    pub files: Vec<MRPackFile>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde()]
pub struct MRPackFile {
    path: String,
    env: Env,
    downloads: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde()]
pub struct Env {
    pub client: EnvSupport,
    pub server: EnvSupport,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde()]
pub enum EnvSupport {
    Required,
    Optional,
    Unsupported,
}

pub async fn import_from_mrpack(server: &mut Server, filename: &str) -> Result<()> {
    let file = File::open(filename)?;

    println!(
        " > {}: {}",
        style("Importing MRPACK").blue(),
        Path::new(filename).file_name().unwrap_or_default().to_string_lossy()
    );

    let mut archive = ZipArchive::new(file)
        .context("reading mrpack zip archive")?;

    let mut mr_index = archive.by_name("modrinth.index.json")?;
    let mut zip_content = Vec::new();
    mr_index.read_to_end(&mut zip_content).context("reading modrinth.index.json from zip file")?;

    println!(
        " > {}",
        style("Reading index...").blue()
    );

    let pack: MRPackIndex = serde_json::from_slice(&zip_content)?;
    import_from_mrpack_index(server, &pack)
        .await.context("importing from mrpack index")?;

    drop(mr_index);

    // really bad code from this point

    // TODO: Overwrites...

    println!(
        " > {}",
        style("Extracting...").blue()
    );

    let tmp_dir = TempDir::new()?;
    archive.extract(&tmp_dir)?;

    // hacky

    bootstrap(&BootstrapContext {
        vars: HashMap::from([
            ("__NO_VARS".to_owned(), "true".to_owned())
        ]),
        output_dir: PathBuf::from("."),
    }, &tmp_dir.path().join("overrides"))
        .context("bootstrap hack (extracting contents to tmp dir) (overrides)")?;

    bootstrap(&BootstrapContext {
        vars: HashMap::from([
            ("__NO_VARS".to_owned(), "true".to_owned())
        ]),
        output_dir: PathBuf::from("."),
    }, &tmp_dir.path().join("server-overrides"))
        .context("bootstrap hack (extracting contents to tmp dir) (server overrides)")?;

    // ! below is an attempt...

/* 
    println!(
        " > {}",
        style("Extracting overrides...").blue()
    );

    let mut server_overrided: Vec<&str> = Vec::new();

    for i in 0..archive.len() {
        let mut entry = &archive.by_index(i)?;
        match entry.enclosed_name() {
            Some(path) => {
                if path.starts_with("overrides") {
                    if server_overrided.contains(&path.to_str().unwrap_or_default()) {
                        drop(entry);
                        continue;
                    };
                } else if path.starts_with("server-overrides") {
                    server_overrided.push(path.to_str().unwrap_or_default());
                } else {
                    println!(
                        " > {}: {}",
                        style("Skipped  ").dim(),
                        style(path.display()).dim()
                    );
                    drop(entry);
                    continue;
                }

                let mut output_path = PathBuf::from(path).strip_prefix("overrides")?;
                fs::create_dir_all(output_path.clone())?;
                let mut outfile = fs::File::create(&output_path)?;

                io::copy(&mut (entry.to_owned()), &mut outfile).context(format!(
                    "Extracting {} to {}",
                    path.display(),
                    output_path.display()
                ))?;

                println!(
                    " > {}: {}",
                    style("Extracted").green(),
                    style(path.display()).dim()
                );

                drop(entry);
            },
            None => {
                drop(entry);
                continue;
            },
        };
    } */

    drop(archive);

    Ok(())
}

pub async fn import_from_mrpack_index(server: &mut Server, index: &MRPackIndex) -> Result<()> {
    for f in &index.files {
        if f.env.client == EnvSupport::Unsupported {
            continue;
        }

        server.import_mrpack(
            f.downloads.first().context("unwrap url from downloads")?)
            .await?;
    }
    Ok(())
}
