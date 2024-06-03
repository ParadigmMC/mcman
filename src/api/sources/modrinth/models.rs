use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::utils::hashing::HashFormat;

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
    String::new()
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
    Unknown,
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
    pub hashes: HashMap<HashFormat, String>,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: u64,
    // file_type omitted
}
