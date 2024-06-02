pub const MRPACK_INDEX_FILE: &str = "modrinth.index.json";

mod mrpack_index;
mod mrpack_file;

use anyhow::Result;
pub use mrpack_index::*;
pub use mrpack_file::*;

use crate::api::{app::App, utils::accessor::Accessor};

use super::Addon;

pub async fn resolve_mrpack_addons(
    app: &App,
    accessor: Accessor,
) -> Result<Vec<Addon>> {
    

    todo!()
}

impl MRPackFile {
    pub async fn into_addon(&self) -> Result<Addon> {
        todo!()
    }
}
