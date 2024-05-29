mod packwiz_mod;
mod packwiz_pack;

use anyhow::Result;
pub use packwiz_pack::*;
pub use packwiz_mod::*;

use super::Addon;

impl PackwizMod {
    pub async fn into_addon(&self) -> Result<Addon> {
        todo!()
    }
}
