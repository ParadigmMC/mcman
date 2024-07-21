use anyhow::Result;

use crate::api::{
    app::App,
    step::{FileMeta, Step},
    utils::url::get_filename_from_url,
};

pub async fn resolve_steps_for_url(
    _app: &App,
    url: impl Into<String>,
    filename: Option<String>,
) -> Result<Vec<Step>> {
    let url: String = url.into();

    let filename = filename.unwrap_or_else(|| get_filename_from_url(&url));

    let metadata = FileMeta {
        cache: None,
        filename,
        ..Default::default()
    };

    Ok(vec![Step::Download {
        url: url.into(),
        metadata,
    }])
}

pub async fn resolve_remove_steps_for_url(
    _app: &App,
    url: impl Into<String>,
    filename: Option<String>,
) -> Result<Vec<Step>> {
    let filename = filename.unwrap_or_else(|| get_filename_from_url(&url.into()));

    Ok(vec![Step::RemoveFile(FileMeta::filename(filename))])
}
