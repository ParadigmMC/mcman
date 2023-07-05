use anyhow::{anyhow, Result};

pub async fn fetch_papermc_versions(
    project: &str,
    client: &reqwest::Client,
) -> Result<Vec<String>> {
    let project = mcapi::papermc::fetch_papermc_project(client, project).await?;

    Ok(project.versions)
}

pub async fn fetch_papermc_builds(
    project: &str,
    version: &str,
    client: &reqwest::Client,
) -> Result<mcapi::papermc::PaperBuildsResponse> {
    Ok(mcapi::papermc::fetch_papermc_builds(
        client,
        project,
        &match version {
            "latest" => {
                let v = fetch_papermc_versions(project, client).await?;

                v.last()
                    .ok_or(anyhow!("Couldn't get latest version"))?
                    .clone()
            }
            id => id.to_owned(),
        },
    )
    .await?)
}

pub async fn fetch_papermc_build(
    project: &str,
    version: &str,
    build: &str,
    client: &reqwest::Client,
) -> Result<mcapi::papermc::PaperVersionBuild> {
    let builds = fetch_papermc_builds(project, version, client).await?;
    Ok(match build {
        "latest" => builds.builds.last(),
        id => builds.builds.iter().find(|&b| b.build.to_string() == id),
    }
    .ok_or(anyhow!(
        "Latest build for project {project} {version} not found"
    ))?
    .clone())
}

pub async fn download_papermc_build(
    project: &str,
    version: &str,
    build_id: &str,
    client: &reqwest::Client,
) -> Result<reqwest::Response> {
    let builds = fetch_papermc_builds(project, version, client).await?;
    let build = match build_id {
        "latest" => builds.builds.last(),
        id => builds.builds.iter().find(|&b| b.build.to_string() == id),
    }
    .ok_or(anyhow!(
        "Build '{build_id}' for project {project} {version} not found"
    ))?
    .clone();
    Ok(
        mcapi::papermc::download_papermc_build(
            client,
            project,
            &builds.version,
            build.build,
            &build.downloads
                .get("application")
                .ok_or(anyhow!("downloads['application'] missing for papermc project {project}/{version}/{build_id}"))?
                .name
        ).await?
    )
}
