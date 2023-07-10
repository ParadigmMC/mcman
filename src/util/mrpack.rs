use anyhow::{Context, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::{io::{Read, Seek}, path::PathBuf, fs};
use zip::ZipArchive;

use crate::{model::Server, downloadable::Downloadable};

#[derive(Debug, Deserialize, Serialize)]
pub struct MRPackIndex {
    pub name: String,
    pub files: Vec<MRPackFile>,
}

impl MRPackIndex {
    pub async fn import_all(
        &self,
        server: &mut Server,
        http_client: &reqwest::Client,
    ) -> Result<()> {
        for f in self.files.iter().filter(|f| f.env.is_none() || f.env.as_ref().unwrap().server != EnvSupport::Unsupported) {
            let url = f.downloads.first().context("unwrap url from downloads")?;
    
            let dl = Downloadable::from_url_interactive(http_client, server, url)
                .await?;
    
            server.mods.push(dl);
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MRPackFile {
    path: String,
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

pub async fn import_from_mrpack<R: Read + Seek>(
    server: &mut Server,
    http_client: &reqwest::Client,
    reader: R,
) -> Result<()> {
    println!(
        " > {}",
        style("Importing mrpack...").cyan(),
    );

    let mut archive = ZipArchive::new(reader).context("reading mrpack zip archive")?;

    println!(" > {}", style("Reading index...").cyan());

    let mut mr_index = archive.by_name("modrinth.index.json")?;
    let mut zip_content = Vec::new();
    mr_index
        .read_to_end(&mut zip_content)
        .context("reading modrinth.index.json from zip file")?;


    let pack: MRPackIndex = serde_json::from_slice(&zip_content)?;
    pack.import_all(server, http_client)
        .await
        .context("importing from mrpack index")?;

    drop(mr_index);

    println!(" > {}", style("Extracting overrides...").cyan());

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

            server.path.join("config").join(path.strip_prefix("overrides")?)
        } else {
            if path.starts_with("server-overrides") {
                server_overrided.push(path.clone());
                server.path.join("config").join(path.strip_prefix("server-overrides")?)
            } else {
                continue;
            }
        };

        queue.push((path, real_path));
    }

    for (path, real_path) in queue {
        let mut zip_file = archive.by_name(&path.to_string_lossy())?;
        let mut zip_content = Vec::new();
        zip_file
            .read_to_end(&mut zip_content)
            .context(format!("reading {} from zip file", path.display()))?;

        fs::create_dir_all(real_path.parent().unwrap())
            .context(format!("Creating parent folder for {}", real_path.display()))?;

        fs::write(&real_path, zip_content)
            .context(format!("Writing {}", real_path.display()))?;

        println!("  => {}", style(path.to_string_lossy()).dim());
    }

    Ok(())
}
