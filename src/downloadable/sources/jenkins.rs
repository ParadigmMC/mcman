//! Bad way pls fix
//!        - dennis

use anyhow::{anyhow, Result};

use crate::util::match_artifact_name;

//pub static API_MAGIC_JOB: &str = "/api/json?tree=url,name,builds[*[url,number,result,artifacts[relativePath,fileName]]]";
static API_MAGIC_JOB: &str = "/api/json?tree=builds[*[url,number,result]]";
static API_MAGIC_BUILD: &str = "/api/json";
static SUCCESS_STR: &str = "SUCCESS";

// has 1 dep, beware lol
pub fn str_process_job(job: &str) -> String {
    job.split('/')
        .map(|j| "/job/".to_owned() + j)
        .collect::<String>()
}

pub async fn get_jenkins_job_value(
    client: &reqwest::Client,
    url: &str,
    job: &str,
) -> Result<serde_json::Value> {
    let base = url.to_owned() + &str_process_job(job) + API_MAGIC_JOB;

    let v: serde_json::Value = client
        .get(&base)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(v)
}

pub async fn get_jenkins_build_value(
    client: &reqwest::Client,
    build_url: &str,
) -> Result<serde_json::Value> {
    let base = build_url.to_owned() + API_MAGIC_BUILD;

    let v: serde_json::Value = client
        .get(&base)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(v)
}

/// returns (`build_url`, fileName, relativePath, `build_number`)
pub async fn get_jenkins_filename(
    client: &reqwest::Client,
    url: &str,
    job: &str,
    build: &str,
    artifact_id: &str,
) -> Result<(String, String, String, i64)> {
    let j = get_jenkins_job_value(client, url, job).await?;

    let mut filtered_builds = j["builds"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|b| b["result"].as_str().unwrap() == SUCCESS_STR);

    let matched_build = match build {
        // iter.first doesnt exist i guess? .next works
        "latest" => filtered_builds.next().unwrap(),
        id => filtered_builds
            .find(|b| b["number"].as_i64().unwrap().to_string() == id)
            .unwrap(),
    };

    let build_url = matched_build["url"].as_str().unwrap();

    let v = get_jenkins_build_value(client, build_url).await?;

    let mut artifacts_iter = v["artifacts"].as_array().unwrap().iter();

    let artifact = match artifact_id {
        "first" => artifacts_iter.next(),
        id => artifacts_iter.find(|a| match_artifact_name(id, a["fileName"].as_str().unwrap())),
    }
    .ok_or(anyhow!(
        "artifact for jenkins build artifact not found ({url};{job};{build};{artifact_id})"
    ))?;

    Ok((
        build_url.to_owned(),
        artifact["fileName"].as_str().unwrap().to_owned(),
        artifact["relativePath"].as_str().unwrap().to_owned(),
        matched_build["number"].as_i64().unwrap(),
    ))
}

pub async fn get_jenkins_download_url(
    client: &reqwest::Client,
    url: &str,
    job: &str,
    build: &str,
    artifact: &str,
) -> Result<String> {
    let (build_url, _, relative_path, _build_number) =
        get_jenkins_filename(client, url, job, build, artifact).await?;

    Ok(build_url + "artifact/" + &relative_path)
}

pub async fn download_jenkins(
    client: &reqwest::Client,
    url: &str,
    job: &str,
    build: &str,
    artifact: &str,
) -> Result<reqwest::Response> {
    let (build_url, _, relative_path, _build_number) =
        get_jenkins_filename(client, url, job, build, artifact).await?;

    let download_url = build_url + "artifact/" + &relative_path;

    Ok(client.get(download_url).send().await?.error_for_status()?)
}

pub async fn fetch_jenkins_description(
    client: &reqwest::Client,
    url: &str,
    job: &str,
) -> Result<String> {
    let base = url.to_owned() + &str_process_job(job) + "/api/json?tree=description";

    let desc = client
        .get(&base)
        .send()
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?["description"]
        .as_str()
        .unwrap()
        .to_owned();

    Ok(desc)
}
