use std::collections::HashMap;

use anyhow::Result;
use serde::de::DeserializeOwned;

use crate::api::{app::App, step::{CacheLocation, FileMeta, Step}, utils::hashing::HashFormat};

mod models;
pub use models::*;

pub const CURSEFORGE_API_URL: &str = "https://api.curse.tools/v1/cf";

pub struct CurseforgeAPI<'a>(pub &'a App);

impl<'a> CurseforgeAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned>(&self, url: String) -> Result<T> {
        self.0
            .http_get_json(format!("{CURSEFORGE_API_URL}/{url}"))
            .await
    }

    pub async fn fetch_mod(&self, mod_id: &str) -> Result<CurseforgeMod> {
        Ok(self.fetch_api::<Data<CurseforgeMod>>(format!("mods/{mod_id}")).await?.data)
    }

    pub async fn fetch_file(&self, mod_id: &str, file_id: &str) -> Result<CurseforgeFile> {
        Ok(self.fetch_api::<Data<CurseforgeFile>>(format!("mods/{mod_id}/files/{file_id}")).await?.data)
    }

    pub async fn fetch_download_url(&self, mod_id: &str, file_id: &str) -> Result<String> {
        Ok(self.fetch_api::<Data<String>>(format!("mods/{mod_id}/files/{file_id}/download-url")).await?.data)
    }

    pub async fn resolve_steps(&self, mod_id: &str, file_id: &str) -> Result<Vec<Step>> {
        let file = self.fetch_file(mod_id, file_id).await?;

        let metadata = FileMeta {
            cache: Some(CacheLocation("curseforge".into(), format!("{mod_id}/{file_id}/{}", file.display_name))),
            filename: file.display_name,
            hashes: convert_hashes(file.hashes),
            size: Some(file.file_length),
            ..Default::default()
        };

        Ok(vec![
            Step::CacheCheck(metadata.clone()),
            Step::Download { url: file.download_url, metadata }
        ])
    }
}

pub fn convert_hashes(hashes: Vec<CurseforgeFileHash>) -> HashMap<HashFormat, String> {
    let mut map = HashMap::new();

    for CurseforgeFileHash { value, algo } in hashes {
        map.insert(algo.into(), value);
    }

    map
}

impl Into<HashFormat> for CurseforgeHashAlgo {
    fn into(self) -> HashFormat {
        match self {
            CurseforgeHashAlgo::Sha1 => HashFormat::Sha1,
            CurseforgeHashAlgo::Md5 => HashFormat::Md5,
        }
    }
}
