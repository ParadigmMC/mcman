pub mod md;
pub mod mrpack;
pub mod packwiz;

use anyhow::{Context, Result};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use tokio::{fs::File, io::BufWriter};

use crate::{downloadable::Downloadable, model::Server};

pub struct SelectItem<T>(pub T, pub String);

impl<T> ToString for SelectItem<T> {
    fn to_string(&self) -> String {
        self.1.clone()
    }
}

pub async fn download_with_progress(
    file: File,
    message: &str,
    downloadable: &Downloadable,
    filename_hint: Option<&str>,
    server: &Server,
    client: &reqwest::Client,
) -> Result<()> {
    let response = downloadable
        .download(server, client, filename_hint)
        .await
        .context("downloadable download")?;
    let progress_bar =
        ProgressBar::with_draw_target(response.content_length(), ProgressDrawTarget::stderr());

    progress_bar.set_message(message.to_owned());
    progress_bar.set_style(
        ProgressStyle::with_template("{msg} [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut file = BufWriter::new(file);
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

pub fn is_default_str(s: &str) -> bool {
    s == "latest"
}

// TODO: better asset matching
//       for example: 'FastAsyncWorldEdit-Bukkit-2.6.1.jar'
//                    'FastAsyncWorldEdit-Bukkit-${mcver}.jar'
//       maybe also support bare asset id? current is asset filename
pub fn match_artifact_name(input: &str, artifact_name: &str) -> bool {
    artifact_name.contains(input)
}
