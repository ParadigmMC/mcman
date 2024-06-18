use anyhow::{anyhow, Result};

pub trait XMLExt {
    fn get_text(&self, k: &str) -> Result<String>;
    fn get_text_all(&self, k: &str) -> Vec<String>;
}

impl XMLExt for roxmltree::Document<'_> {
    fn get_text(&self, k: &str) -> Result<String> {
        self.descendants()
            .find_map(|elem| {
                if elem.tag_name().name() == k {
                    Some(elem.text()?.to_owned())
                } else {
                    None
                }
            })
            .ok_or(anyhow!("XML element not found: {}", k))
    }

    fn get_text_all(&self, k: &str) -> Vec<String> {
        self.descendants()
            .filter_map(|t| {
                if t.tag_name().name() == k {
                    Some(t.text()?.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MavenMetadata {
    pub latest: Option<String>,
    pub group_id: Option<String>,
    pub artifact_id: Option<String>,
    pub versions: Vec<String>,
}

impl MavenMetadata {
    pub fn find_url(&self, url: &str) -> Option<(String, String)> {
        let t = url.split_once(&format!(
            "{}/{}",
            self.group_id.clone()?.replace(['.', ':'], "/"),
            self.artifact_id.clone()?
        ))?;
        Some((t.0.to_owned(), t.1.to_owned()))
    }
}
