use std::time::Duration;

use anyhow::Result;
use tokio::time::sleep;

pub trait ModrinthWaitRatelimit<T> {
    async fn wait_ratelimit(self) -> Result<T>;
}

impl ModrinthWaitRatelimit<reqwest::Response> for reqwest::Response {
    async fn wait_ratelimit(self) -> Result<Self> {
        let res = if let Some(h) = self.headers().get("x-ratelimit-remaining") {
            if String::from_utf8_lossy(h.as_bytes()) == "1" {
                let ratelimit_reset =
                    String::from_utf8_lossy(self.headers()["x-ratelimit-reset"].as_bytes())
                        .parse::<u64>()?;
                let amount = ratelimit_reset;
                println!(" (!) Ratelimit exceeded. sleeping for {amount} seconds...");
                sleep(Duration::from_secs(amount)).await;
            }
            self
        } else {
            self.error_for_status()?
        };

        Ok(res)
    }
}
