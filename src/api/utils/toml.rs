use std::{io::Write, path::{Path, PathBuf}};

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

use super::fs::create_parents_sync;

pub fn try_find_toml_upwards<T: DeserializeOwned>(filename: &str) -> Result<Option<(PathBuf, T)>> {
    let mut path = std::env::current_dir()?;

    let found_path = loop {
        path.push(filename);

        if path.is_file() {
            break path;
        }

        if !(path.pop() && path.pop()) {
            return Ok(None);
        }
    };

    read_toml(&found_path).map(|data| Some((found_path, data)))
}

pub fn read_toml<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let data: T = toml::from_str(&std::fs::read_to_string(&path)?)?;

    Ok(data)
}

pub fn write_toml<T: Serialize>(path: &Path, filename: &str, value: &T) -> Result<()> {
    create_parents_sync(path)?;

    let content = toml::to_string_pretty(value)?;

    let mut file = std::fs::File::open(path.join(filename))?;
    file.write_all(content.as_bytes())?;

    Ok(())
}
