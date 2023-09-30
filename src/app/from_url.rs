use anyhow::Result;

use crate::{App, model::Downloadable};

impl App {
    pub async fn dl_from_url(
        &self,
        url: &str
    ) -> Result<Downloadable> {
        todo!()
    }
}