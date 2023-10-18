use anyhow::Result;

use crate::model::Downloadable;

use super::App;

impl App {
    pub async fn dl_from_string(
        &self,
        s: &str
    ) -> Result<Downloadable> {
        if s.starts_with("http") {
            Ok(self.dl_from_url(s).await?)
        } else {
            todo!()
        }
    }

    pub async fn dl_from_url(
        &self,
        url: &str
    ) -> Result<Downloadable> {
        todo!()
    }
}
