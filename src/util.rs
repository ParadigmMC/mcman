use std::path::Path;

use futures::StreamExt;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use tokio::{fs::File, io::BufWriter};

use crate::error::{CliError, Result};

pub async fn download_with_progress(
    dir: &Path,
    filename: &str,
    response: reqwest::Response,
) -> Result<()> {
    let progress_bar =
        ProgressBar::with_draw_target(response.content_length(), ProgressDrawTarget::stderr());

    progress_bar.set_message(filename.to_owned());
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg} [{wide_bar}] {bytes_per_sec}")
            .map_err(CliError::Indicatif)?
            .progress_chars("=> "),
    );

    let mut file = BufWriter::new(File::create(dir.join(filename)).await?);
    let mut bytes_downloaded = 0;

    let mut stream = response.bytes_stream();
    while let Some(item) = stream.next().await {
        let item = item?;
        tokio::io::copy(&mut item.as_ref(), &mut file).await?;

        bytes_downloaded += item.len();
        progress_bar.set_position(bytes_downloaded as u64);
    }

    Ok(())
}
