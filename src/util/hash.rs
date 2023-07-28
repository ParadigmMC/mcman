use std::path::PathBuf;

use anyhow::Result;
use futures::StreamExt;
use sha2::{Digest, Sha256};

pub fn hash_contents(contents: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(contents);

    // unholy hell
    let hash = (hasher.finalize().as_slice() as &[u8])
        .iter()
        .map(|b| format!("{b:x?}"))
        .collect::<String>();

    hash
}

pub fn hash_file(path: &PathBuf) -> Result<String> {
    let mut hasher = Sha256::new();

    let mut file = std::fs::File::open(path)?;

    std::io::copy(&mut file, &mut hasher)?;

    // unholy hell
    let hash = (hasher.finalize().as_slice() as &[u8])
        .iter()
        .map(|b| format!("{b:x?}"))
        .collect::<String>();

    Ok(hash)
}

pub async fn get_hash_url(client: &reqwest::Client, url: &str) -> Result<String> {
    // rust-analyzer broke
    let mut hasher = Sha256::new();

    let mut stream = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .bytes_stream();

    while let Some(item) = stream.next().await {
        let item = item?;
        hasher.update(item);
    }

    // unholy hell
    let hash = (hasher.finalize().as_slice() as &[u8])
        .iter()
        .map(|b| format!("{b:x?}"))
        .collect::<String>();

    Ok(hash)
}
