use std::{ffi::OsString, fs::DirEntry, io::{Read, Seek}, path::PathBuf};

use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use zip::ZipArchive;

use crate::api::app::App;

pub trait ReadSeek: std::io::Read + Seek {}

pub enum Accessor {
    Local(PathBuf),
    Remote(reqwest::Url),
    Zip(ZipArchive<Box<dyn ReadSeek>>),
}

impl Accessor {
    pub async fn dir(&self) -> Result<Vec<String>> {
        match self {
            Accessor::Zip(zip) => Ok(zip.file_names().map(ToOwned::to_owned).collect()),
            Accessor::Local(path) => Ok(path.read_dir()?
                .filter_map(|r| r.ok())
                .map(|n| n.file_name().to_string_lossy().into_owned())
                .collect()),
            Accessor::Remote(_) => Err(anyhow!("cannot dir() Accessor::Remote")),
        }
    }

    pub async fn json<T: DeserializeOwned>(&mut self, app: &App, path: &str) -> Result<T> {
        match self {
            Accessor::Local(base) => Ok(serde_json::from_reader(std::fs::File::open(base.join(path))?)?),
            Accessor::Zip(zip) => Ok(serde_json::from_reader(zip.by_name(path)?)?),
            Accessor::Remote(url) => Ok(app.http_get_json(url.join(path)?).await?),
        }
    }
}
