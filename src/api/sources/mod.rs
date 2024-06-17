use std::collections::HashMap;

use anyhow::Result;

use super::{app::App, step::{CacheStrategy, Step}, utils::url::get_filename_from_url};

pub mod vanilla;
pub mod modrinth;

pub async fn resolve_steps_for_url(
    app: &App,
    url: impl Into<String>,
    filename: Option<String>,
) -> Result<Vec<Step>> {
    let url: String = url.into();

    let filename = filename.unwrap_or_else(|| {
        get_filename_from_url(&url)
    });

    Ok(vec![
        Step::CacheCheck(CacheStrategy::Indexed {
            namespace: "url".into(),
            path: None,
            key: url.clone(),
        }),
        Step::Download {
            url: url.into(),
            filename,
            size: None,
            hashes: HashMap::new(),
        }
    ])
}
