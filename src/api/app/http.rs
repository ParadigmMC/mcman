use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use reqwest::{IntoUrl, Response};
use serde::de::DeserializeOwned;
use tokio::time::sleep;

use super::App;

impl App {
    pub async fn http_get_with<F: FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder>(
        &self,
        url: impl IntoUrl,
        f: F,
    ) -> Result<Response> {
        log::trace!("GET {}", url.as_str());

        let req = self.http_client.get(url.as_str());

        let req = f(req);

        let res = req.send().await?.error_for_status()?;

        if res
            .headers()
            .get("x-ratelimit-remaining")
            .is_some_and(|x| String::from_utf8_lossy(x.as_bytes()) == "1")
        {
            log::info!("Hit ratelimit");

            let ratelimit_reset =
                String::from_utf8_lossy(res.headers()["x-ratelimit-reset"].as_bytes())
                    .parse::<u64>()?;
            let sleep_amount = match url.into_url()?.domain() {
                Some("github.com") => {
                    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                    let amount = ratelimit_reset - now;
                    Some(amount)
                },
                Some("modrinth.com") => Some(ratelimit_reset),
                _ => None,
            };

            if let Some(amount) = sleep_amount {
                sleep(Duration::from_secs(amount)).await;
            }
        }

        Ok(res)
    }

    pub async fn http_get(&self, url: impl IntoUrl) -> Result<Response> {
        self.http_get_with(url, |x| x).await
    }

    pub async fn http_get_json<T: DeserializeOwned>(&self, url: impl IntoUrl) -> Result<T> {
        self.http_get_json_with(url, |x| x).await
    }

    pub async fn http_get_json_with<
        T: DeserializeOwned,
        F: FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
    >(
        &self,
        url: impl IntoUrl,
        f: F,
    ) -> Result<T> {
        let res = self.http_get_with(url, f).await?;

        let full = res.bytes().await?;

        serde_json::from_slice(&full)
            .with_context(|| format!("JSON parsing error: {}", String::from_utf8_lossy(&full)))
    }
}
