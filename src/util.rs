use std::path::Path;

use anyhow::{Context, Result};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use tokio::{fs::File, io::BufWriter};

use crate::{downloadable::Downloadable, model::Server};

pub async fn download_with_progress(
    dir: &Path,
    filename: &str,
    downloadable: &Downloadable,
    server: &Server,
    client: &reqwest::Client,
) -> Result<()> {
    let response = downloadable.download(server, client).await.context("downloadable download")?;
    let progress_bar =
        ProgressBar::with_draw_target(response.content_length(), ProgressDrawTarget::stderr());

    progress_bar.set_message(filename.to_owned());
    progress_bar.set_style(
        ProgressStyle::with_template("{msg} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut file = BufWriter::new(
        File::create(dir.join(filename))
            .await
            .context("Failed to create output file")?,
    );
    let mut bytes_downloaded = 0;

    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let item = item?;
        tokio::io::copy(&mut item.as_ref(), &mut file).await?;

        bytes_downloaded += item.len();
        progress_bar.set_position(bytes_downloaded as u64);
    }

    progress_bar.finish_and_clear();

    Ok(())
}

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub fn is_default_str(s: &String) -> bool {
    s == "default"
}
