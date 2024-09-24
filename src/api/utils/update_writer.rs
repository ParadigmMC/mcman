use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use toml_edit::DocumentMut;
use zip::ZipArchive;

use crate::api::{
    app::App,
    models::{
        mrpack::MRPackIndex,
        packwiz::{PackwizPack, PackwizPackIndex},
        ModpackSource, ModpackType, Source, SourceType,
    },
};

use super::toml::read_toml;

/// An `Accessor` allows for filesystem, remote or zip file access.
pub enum UpdateWriter {
    File(PathBuf, DocumentMut),
    MRPack(PathBuf, ZipArchive<std::fs::File>, MRPackIndex),
    Packwiz(PathBuf, PackwizPack, PackwizPackIndex),
}

impl UpdateWriter {
    pub fn from(source: &Source, relative_to: &Path) -> Result<Self> {
        todo!();

        /* match &source.source_type {
            SourceType::File { path } => Self::File(path, read_toml(relative_to.join(path).to_path_buf())?)),
            SourceType::Folder { path } => unimplemented!(),
            SourceType::Modpack {
                modpack_source: ModpackSource::Local { path, can_update: true },
                modpack_type
            } => Ok(match modpack_type {
                ModpackType::MRPack => Self::MRPack(
                    relative_to.join(path).to_path_buf(),
                    ZipArchive::new(std::fs::File::open(relative_to.join(path))?)?,
                    todo!()
                ),
                ModpackType::Packwiz => Self::Packwiz(
                    relative_to.join(path),
                    todo!(),
                    todo!(),
                ),
                ModpackType::Unsup => unimplemented!(),
            }),
            _ => Err(anyhow!("Can't make an UpdateWriter")),
        } */
    }
}
