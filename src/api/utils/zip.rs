use std::{io::{Read, Seek, Write}, path::Path};

use anyhow::{anyhow, Context, Result};
use walkdir::WalkDir;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

use crate::api::app::APP_VERSION;

use super::{fs::create_parents_sync, pathdiff::DiffTo};

pub async fn unzip<T: Read + Seek>(reader: T, to: &Path, prefix: Option<String>) -> Result<()> {
    let mut archive = ZipArchive::new(reader)?;

    let mut files = archive.file_names().map(ToOwned::to_owned).collect::<Vec<_>>();

    if let Some(prefix) = prefix.map(|p| format!("{p}/")) {
        if files.iter().all(|f| f.starts_with(&prefix)) {
            files = files.into_iter()
                .map(|f| f.replacen(&prefix, "", 1))
                .collect()
        }
    }


    for filename in files {
        if filename.ends_with('/') {
            // directory
            continue;
        }

        let mut file = archive.by_name(&filename)?;
        let target_path = to.join(&filename);

        create_parents_sync(&target_path)?;
        let mut target_file = std::fs::File::create(&target_path)?;

        std::io::copy(&mut file, &mut target_file)?;
    }
    
    Ok(())
}

pub async fn zip<T: Write + Seek>(writer: T, folder: &Path) -> Result<()> {
    let mut archive = ZipWriter::new(writer);

    archive.set_comment(format!("generated by mcman/{APP_VERSION}"));

    for entry in WalkDir::new(folder) {
        let entry = entry.with_context(|| "WalkDir")?;

        let path = folder.try_diff_to(entry.path())?;
        let path = path.to_string_lossy();

        if entry.file_type().is_dir() {
            archive.add_directory(path, FileOptions::default())?;
            continue;
        }

        archive.start_file(path, FileOptions::default())?;

        let mut file = std::fs::File::open(entry.path())?;
        std::io::copy(&mut file, &mut archive)?;
    }

    Ok(())
}
