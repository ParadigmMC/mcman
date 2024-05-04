use anyhow::{Context, Result};

pub const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " - ",
    env!("CARGO_PKG_REPOSITORY"),
);

pub struct App {
    http_client: reqwest::Client,
}

impl App {
    pub fn new() -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .context("Initializing http_client")?;

        Ok(Self {
            http_client,
        })
    }
}
