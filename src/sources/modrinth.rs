use std::collections::HashMap;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::{App, ResolvedFile, CacheStrategy};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModrinthProject {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub client_side: DependencyType,
    pub server_side: DependencyType,
    //pub body: String,
    pub project_type: String,
    // ...
    #[serde(default = "empty")]
    pub id: String,
    //pub team: String,
    pub versions: Vec<String>,
}

fn empty() -> String {
    String::from("")
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModrinthSearchResults {
    pub hits: Vec<ModrinthProject>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModrinthVersion {
    pub name: String,
    pub version_number: String,
    pub changelog: String,
    pub dependencies: Vec<ModrinthDependency>,
    pub game_versions: Vec<String>,
    pub version_type: VersionType,
    pub loaders: Vec<String>,
    pub featured: bool,
    pub status: ModrinthStatus,
    pub requested_status: Option<ModrinthStatus>,
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub date_published: String,
    pub downloads: i64,
    pub files: Vec<ModrinthFile>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModrinthDependency {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub file_name: Option<String>,
    pub dependency_type: Option<DependencyType>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
    Unsupported,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Release,
    Beta,
    Alpha,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ModrinthStatus {
    Listed,
    Archived,
    Draft,
    Unlisted,
    Scheduled,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ModrinthFile {
    pub hashes: HashMap<String, String>,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i32,
    // file_type omitted
}

pub struct ModrinthAPI<'a>(pub &'a App);

static API_URL: &str = "https://api.modrinth.com/v2";

impl<'a> ModrinthAPI<'a> {
    pub async fn fetch_api<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let json: T = self.0.http_client.get(url).send().await?.error_for_status()?.json().await?;
        
        Ok(json)
    }

    pub async fn fetch_project(&self, id: &str) -> Result<ModrinthProject> {
        self.fetch_api(&format!("{API_URL}/project/{id}")).await
    }

    pub async fn fetch_all_versions(&self, id: &str) -> Result<Vec<ModrinthVersion>> {
        self.fetch_api(&format!("{API_URL}/project/{id}/version")).await
    }

    pub async fn fetch_versions(&self, id: &str) -> Result<Vec<ModrinthVersion>> {
        let versions = self.fetch_all_versions(id).await?;

        Ok(self.0.server.filter_modrinth_versions(&versions))
    }

    pub async fn fetch_version(&self, id: &str, version: &str) -> Result<ModrinthVersion> {
        let versions = self.fetch_versions(id).await?;

        let ver = version.replace("${mcver}", &self.0.mc_version());
        let ver = ver.replace("${mcversion}", &self.0.mc_version());

        let version_data = match ver.as_str() {
            "latest" => versions.first(),
            ver =>  versions.iter().find(|v| v.id == ver || v.name == ver || v.version_number == ver)
        }.ok_or(anyhow!("Couln't find version '{ver}' ('{version}') for Modrinth project '{id}'"))?.clone();

        Ok(version_data)
    }

    pub async fn fetch_file(&self, id: &str, version: &str) -> Result<(ModrinthFile, ModrinthVersion)> {
        let version = self.fetch_version(id, version).await?;

        Ok((
            version.files.iter().find(|f| f.primary)
                .ok_or(anyhow!("No primary file found on modrinth:{id}/{} ({})", version.id, version.name))?.clone(),
            version
        ))
    }

    pub async fn search(&self, query: &str) -> Result<Vec<ModrinthProject>> {
        Ok(self.0.http_client.get(format!("{API_URL}/search"))
            .query(&[("query", query), ("facets", &self.0.server.jar.get_modrinth_facets(&self.0.mc_version())?)])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn resolve_source(&self, id: &str, version: &str) -> Result<ResolvedFile> {
        let (file, version) = self.fetch_file(id, version).await?;

        let cached_file_path = format!("{id}/{}/{}", version.id, file.filename);

        Ok(ResolvedFile {
            url: file.url,
            filename: file.filename,
            cache: CacheStrategy::File {
                namespace: String::from("modrinth"),
                path: cached_file_path,
            },
            size: Some(file.size),
            hashes: file.hashes,
        })
    }
}
