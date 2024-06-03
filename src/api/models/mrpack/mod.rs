use anyhow::Result;

mod mrpack_index;
mod mrpack_file;

pub use mrpack_index::*;
pub use mrpack_file::*;

pub const MRPACK_INDEX_FILE: &str = "modrinth.index.json";

use crate::api::{app::App, utils::accessor::Accessor};

use super::Addon;

pub async fn resolve_mrpack_addons(
    app: &App,
    mut accessor: Accessor,
) -> Result<Vec<Addon>> {
    let mut addons = vec![];

    let index: MRPackIndex = accessor.json(app, MRPACK_INDEX_FILE).await?;

    for file in index.files {
        addons.push(file.into_addon().await?);
    }

    Ok(addons)
}

impl MRPackFile {
    pub async fn into_addon(&self) -> Result<Addon> {
        todo!()
    }
}
