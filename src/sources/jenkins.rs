//! Bad way pls fix
//!        - dennis

use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::app::{App, CacheStrategy, ResolvedFile};

//pub static API_MAGIC_JOB: &str = "/api/json?tree=url,name,builds[*[url,number,result,artifacts[relativePath,fileName]]]";
static API_MAGIC_JOB: &str = "/api/json?tree=builds[*[url,number,result]]";
static API_MAGIC_BUILD: &str = "/api/json";
static SUCCESS_STR: &str = "SUCCESS";

pub async fn resolve_source(
    app: &App,
    url: &str,
    job: &str,
    build: &str,
    artifact: &str,
) -> Result<ResolvedFile> {
    let (build_url, filename, relative_path, build_number, md5hash) =
        get_jenkins_filename(&app.http_client, url, job, build, artifact).await?;

    // ci.luckto.me => ci-lucko-me
    let folder = url.replace("https://", "");
    let folder = folder.replace("http://", "");
    let folder = folder.replace("/", " ");
    let folder = folder.trim();
    let folder = folder.replace(" ", "-");

    let cached_file_path = format!("{folder}/{job}/{build_number}/{filename}");

    Ok(ResolvedFile {
        url: format!("{build_url}artifact/{relative_path}"),
        filename,
        cache: CacheStrategy::File {
            namespace: String::from("jenkins"),
            path: cached_file_path,
        },
        size: None,
        hashes: if let Some(md5) = md5hash {
            HashMap::from([("md5".to_owned(), md5.clone())])
        } else {
            HashMap::new()
        },
    })
}

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

/// returns (`build_url`, fileName, relativePath, `build_number`, md5)
pub async fn get_jenkins_filename(
    client: &reqwest::Client,
    url: &str,
    job: &str,
    build: &str,
    artifact_id: &str,
) -> Result<(String, String, String, i64, Option<String>)> {
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
        id => {
            let id = id.replace("${build}", &matched_build["number"].as_i64().unwrap().to_string());

            artifacts_iter.find(|a| a["fileName"].as_str().unwrap() == id)
                .or(artifacts_iter.find(|a| id.contains(a["fileName"].as_str().unwrap())))
        },
    }
    .ok_or(anyhow!(
        "artifact for jenkins build artifact not found ({url};{job};{build};{artifact_id})"
    ))?;

    let md5hash = if let Some(serde_json::Value::Array(values)) = matched_build.get("fingerprint") {
        values
            .iter()
            .find(|v| v["fileName"].as_str().unwrap() == artifact["fileName"].as_str().unwrap())
            .map(|v| v["hash"].as_str().unwrap().to_owned())
    } else {
        None
    };

    Ok((
        build_url.to_owned(),
        artifact["fileName"].as_str().unwrap().to_owned(),
        artifact["relativePath"].as_str().unwrap().to_owned(),
        matched_build["number"].as_i64().unwrap(),
        md5hash,
    ))
}

pub async fn get_jenkins_download_url(
    client: &reqwest::Client,
    url: &str,
    job: &str,
    build: &str,
    artifact: &str,
) -> Result<String> {
    let (build_url, _, relative_path, _build_number, _md5) =
        get_jenkins_filename(client, url, job, build, artifact).await?;

    Ok(build_url + "artifact/" + &relative_path)
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
