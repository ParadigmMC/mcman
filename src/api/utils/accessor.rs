use std::{fmt, fs::File, path::PathBuf};
use tokio::fs;

use anyhow::{anyhow, Result};
use reqwest::Url;
use serde::de::DeserializeOwned;
use zip::ZipArchive;

use crate::api::app::App;

/// An `Accessor` allows for filesystem, remote or zip file access.
pub enum Accessor {
    Local(PathBuf),
    Remote(reqwest::Url),
    ZipLocal((PathBuf, ZipArchive<File>)),
    //ZipRemote(SomeSortOfTempFile, ZipArchive<File>),
}

impl fmt::Display for Accessor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Local(path) | Self::ZipLocal((path, _)) => {
                write!(f, "{}", path.to_string_lossy())
            },
            Self::Remote(url) => url.fmt(f),
        }
    }
}

impl Accessor {
    pub fn from(str: &str) -> Result<Self> {
        if str.starts_with("http://") || str.starts_with("https://") {
            Ok(Self::Remote(Url::parse(str)?))
        } else if str.ends_with(".zip") || str.ends_with(".mrpack") {
            let file = File::open(str)?;
            let archive = ZipArchive::new(file)?;
            Ok(Self::ZipLocal((PathBuf::from(str), archive)))
        } else {
            Ok(Self::Local(PathBuf::from(str)))
        }
    }

    /// Try listing a directory
    #[allow(unused)]
    pub async fn dir(&self) -> Result<Vec<String>> {
        match self {
            Self::ZipLocal((_, zip)) => Ok(zip.file_names().map(ToOwned::to_owned).collect()),
            Self::Local(path) => Ok(path
                .read_dir()?
                .filter_map(std::result::Result::ok)
                .map(|n| n.file_name().to_string_lossy().into_owned())
                .collect()),
            Self::Remote(_) => Err(anyhow!("cannot dir() Accessor::Remote")),
        }
    }

    /// Read a JSON file
    pub async fn json<T: DeserializeOwned>(&mut self, app: &App, path: &str) -> Result<T> {
        Ok(match self {
            Self::Local(base) => serde_json::from_reader(File::open(base.join(path))?)?,
            Self::ZipLocal((_, zip)) => serde_json::from_reader(zip.by_name(path)?)?,
            Self::Remote(url) => app.http_get_json(url.join(path)?).await?,
        })
    }

    /// Read a TOML file
    pub async fn toml<T: DeserializeOwned>(&mut self, app: &App, path: &str) -> Result<T> {
        Ok(match self {
            Self::Local(base) => toml::from_str(&fs::read_to_string(base.join(path)).await?)?,
            Self::ZipLocal((_, zip)) => {
                let file = zip.by_name(path)?;

                toml::from_str(&std::io::read_to_string(file)?)?
            },
            Self::Remote(url) => {
                let res = app.http_get(url.join(path)?).await?;
                let content = res.text().await?;

                toml::from_str(&content)?
            },
        })
    }
}
