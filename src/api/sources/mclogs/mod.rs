use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::api::app::App;

mod models;
pub use models::*;

const API_V1: &str = "https://api.mclo.gs/1";

pub struct MCLogsAPI<'a>(pub &'a App);

impl<'a> MCLogsAPI<'a> {
    pub async fn paste_log(&self, content: &str) -> Result<LogFileMetadata> {
        let params = HashMap::from([("content", content)]);

        let json = self
            .0
            .http_client
            .post(format!("{API_V1}/log"))
            .form(&params)
            .send()
            .await?
            .error_for_status()?
            .json::<MaybeSuccess<LogFileMetadata>>()
            .await?;

        json.into()
    }

    #[allow(unused)]
    pub async fn fetch_insights(&self, id: &str) -> Result<LogInsights> {
        let json = self
            .0
            .http_get_json::<MaybeSuccess<LogInsights>>(format!("{API_V1}/insights/{id}"))
            .await?;

        json.into()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MaybeSuccess<T> {
    Error {
        error: String,
    },
    Success {
        #[serde(flatten)]
        value: T,
    },
}

impl<T> From<MaybeSuccess<T>> for Result<T> {
    fn from(val: MaybeSuccess<T>) -> Self {
        match val {
            MaybeSuccess::Success { value } => Ok(value),
            MaybeSuccess::Error { error } => Err(anyhow!(error)),
        }
    }
}
