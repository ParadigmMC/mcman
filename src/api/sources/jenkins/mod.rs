use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};

mod models;
pub use models::*;

use crate::api::{app::App, step::{CacheLocation, FileMeta, Step}, utils::{hashing::HashFormat, url::url_to_folder}};

static SUCCESS_STR: &str = "SUCCESS";

pub fn jenkins_job_url(url: &str, job: &str) -> String {
    job.split('/')
        .fold(url.strip_suffix('/').unwrap_or(url).to_owned(), |acc, j| {
            format!("{acc}/job/{j}")
        })
}

pub struct JenkinsAPI<'a>(pub &'a App);

impl<'a> JenkinsAPI<'a> {
    pub async fn fetch_builds(&self, url: &str, job: &str) -> Result<Vec<JenkinsBuild>> {
        Ok(self
            .0
            .http_get_json::<JenkinsBuildsResponse>(format!(
                "{}/api/json?tree=builds[*[url,number,result]]",
                jenkins_job_url(url, job)
            ))
            .await?
            .builds)
    }

    pub async fn fetch_build(&self, url: &str, job: &str, build: &str) -> Result<JenkinsBuild> {
        let builds = self
            .fetch_builds(url, job)
            .await
            .context("Fetching jenkins builds")?;
        let builds = builds
            .into_iter()
            .filter(|b| b.result == SUCCESS_STR)
            .collect::<Vec<_>>();

        let selected_build = match build {
            "latest" => builds.first(),
            id => builds.iter().find(|b| b.number.to_string() == id),
        }
        .ok_or(anyhow!(
            "Can't find Jenkins build '{build}', URL: '{url}', Job: '{job}'"
        ))?
        .clone();

        Ok(selected_build)
    }

    pub async fn fetch_artifacts(&self, build_url: &str) -> Result<Vec<JenkinsArtifact>> {
        Ok(self
            .0
            .http_get_json::<JenkinsArtifactsResponse>(format!(
                "{build_url}/api/json?tree=artifacts[*]"
            ))
            .await?
            .artifacts)
    }

    // TODO: refactorize
    pub async fn fetch_artifact(
        &self,
        url: &str,
        job: &str,
        build: &str,
        artifact: &str,
    ) -> Result<(JenkinsBuild, JenkinsArtifact)> {
        let selected_build = self
            .fetch_build(url, job, build)
            .await
            .context("Fetching jenkins build")?;
        let artifacts = self
            .fetch_artifacts(&selected_build.url)
            .await
            .context("Fetching jenkins artifacts")?;

        let artifact = artifact.replace("${build}", &selected_build.number.to_string());

        let selected_artifact = match artifact.as_str() {
            "first" => artifacts.first(),
            id => artifacts.iter().find(|a| a.file_name == id)
                .or_else(|| artifacts.iter().find(|a| a.file_name.contains(id))),
        }.ok_or(anyhow!(
            "Can't find Jenkins artifact '{artifact}', on build '{}' ({build}), job '{job}', url: '{url}'",
            selected_build.number
        ))?.clone();

        Ok((selected_build, selected_artifact))
    }

    pub async fn resolve_steps(
        &self,
        url: &str,
        job: &str,
        build: &str,
        artifact: &str,
        custom_filename: Option<String>,
    ) -> Result<Vec<Step>> {
        let (build, artifact) = self
            .fetch_artifact(url, job, build, artifact)
            .await
            .context("Fetching jenkins artifact")?;

        let mut hashes = HashMap::new();

        if let Some(JenkinsFingerprint { hash, .. }) = build
            .fingerprint
            .iter()
            .find(|f| f.file_name == artifact.file_name)
        {
            hashes.insert(HashFormat::Md5, hash.clone());
        }

        let metadata = FileMeta {
            cache: Some(CacheLocation("jenkins".into(), format!(
                "{}/{}/{}/{}",
                url_to_folder(url),
                job,
                build.number,
                artifact.file_name,
            ))),
            hashes,
            filename: custom_filename.unwrap_or(artifact.file_name)
                .replace("${build}", build.number.to_string().as_str()),
            ..Default::default()
        };

        let url = format!("{}artifact/{}", build.url, artifact.relative_path);

        Ok(vec![
            Step::CacheCheck(metadata.clone()),
            Step::Download { url, metadata },
        ])
    }

    pub async fn fetch_description(&self, url: &str, job: &str) -> Result<Option<String>> {
        Ok(self
            .0
            .http_get_json::<JenkinsJobResponse>(format!(
                "{}/api/json?tree=description",
                jenkins_job_url(url, job)
            ))
            .await?
            .description)
    }
}
