use anyhow::Result;

use crate::model::Downloadable;

use super::App;

impl App {
    pub async fn dl_from_url(
        &self,
        url: &str
    ) -> Result<Downloadable> {
        todo!()
    }
}
