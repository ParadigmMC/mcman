use anyhow::{Result, anyhow};

pub fn get_metadata_url(url: &str, group_id: &str, artifact_id: &str) -> String {
    format!(
        "{url}/{}/{artifact_id}/maven-metadata.xml",
        group_id.replace('.', "/")
    )
}

pub async fn get_maven_versions(
    client: &reqwest::Client,
    url: &str,
    group_id: &str,
    artifact_id: &str,
) -> Result<(String, Vec<String>)> {
    let xml = client
        .get(get_metadata_url(url, group_id, artifact_id))
        .send()
        .await?
        .text()
        .await?;

    let doc = roxmltree::Document::parse(&xml)?;

    let latest = doc.descendants().find_map(|t| {
        if t.tag_name().name() == "latest" {
            Some(t.text()?.to_owned())
        } else {
            None
        }
    });

    let list = doc
        .descendants()
        .filter_map(|t| {
            if t.tag_name().name() == "version" {
                Some(t.text()?.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok((
        latest.unwrap_or_else(|| list.first().cloned().unwrap_or_default()),
        list,
    ))
}

pub async fn get_maven_url(
    client: &reqwest::Client,
    url: &str,
    group_id: &str,
    artifact_id: &str,
    version: &str,
    file: &str,
    mcver: &str,
) -> Result<String> {
    let version = match_maven_version(client, url, group_id, artifact_id, version, mcver).await?;

    Ok(format!(
        "{url}/{}/{artifact_id}/{version}/{}",
        group_id.replace('.', "/"),
        get_maven_filename(client, url, group_id, artifact_id, &version, file, mcver).await?
    ))
}

pub async fn get_maven_filename(
    client: &reqwest::Client,
    url: &str,
    group_id: &str,
    artifact_id: &str,
    version: &str,
    file: &str,
    mcver: &str,
) -> Result<String> {
    let version = match_maven_version(client, url, group_id, artifact_id, version, mcver).await?;

    let file = file.replace("${artifact}", artifact_id)
        .replace("${version}", &version)
        .replace("${mcversion}", mcver)
        .replace("${mcver}", mcver);

    Ok(if file.contains('.') {
        file
    } else {
        file + ".jar"
    })
}

pub async fn match_maven_version(
    client: &reqwest::Client,
    url: &str,
    group_id: &str,
    artifact_id: &str,
    version: &str,
    mcver: &str,
) -> Result<String> {
    let fetch_versions = || get_maven_versions(client, url, group_id, artifact_id);

    let version = match version {
        "latest" => fetch_versions().await?.0,
        id => if id.contains("$") {
            let versions = fetch_versions().await?.1;
            let id = id.replace("${artifact}", artifact_id)
                .replace("${mcversion}", mcver)
                .replace("${mcver}", mcver);
            versions.iter().find(|v| {
                v.to_owned() == &id
            }).or_else(|| {
                versions.iter().find(|v| {
                    v.contains(&id)
                })
            }).ok_or(anyhow!("Couldn't resolve maven artifact version"))?
            .to_owned()
        } else {
            id.to_owned()
        },
    };

    Ok(version)
}
