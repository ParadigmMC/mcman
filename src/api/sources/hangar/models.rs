use std::collections::{HashMap, HashSet};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Namespace {
    pub owner: String,
    pub slug: String,
}

impl ToString for Namespace {
    fn to_string(&self) -> String {
        format!("{}/{}", self.owner, self.slug)
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStats {
    pub views: i64,
    pub downloads: i64,
    pub recent_views: i64,
    pub recent_downloads: i64,
    pub stars: i64,
    pub watchers: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VersionStats {
    pub total_downloads: i64,
    pub platform_downloads: HashMap<Platform, i64>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    AdminTools,
    Chat,
    DevTools,
    Economy,
    Gameplay,
    Games,
    Protection,
    RolePlaying,
    WorldManagement,
    Misc,
    #[default]
    Undefined,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    #[default]
    Public,
    New,
    NeedsChanges,
    NeedsApproval,
    SoftDelete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub created_at: String,
    pub name: String,
    pub namespace: Namespace,
    pub stats: ProjectStats,
    pub category: Category,
    pub last_updated: String,
    pub visibility: Visibility,
    pub avatar_url: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSettings {
    pub links: Vec<LinkSection>,
    pub tags: Vec<String>,
    pub license: ProjectLicense,
    pub keywords: Vec<String>,
    pub sponsors: String,
    pub donation: ProjectDonation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDonation {
    pub enable: bool,
    pub subject: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectLicense {
    pub name: String,
    pub url: String,
    #[serde(rename = "type")]
    pub license_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkSection {
    pub id: i64,
    pub link_type: LinkType,
    pub title: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum LinkType {
    Top,
    Sidebar,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    pub id: i64,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectVersion {
    pub created_at: String,
    pub name: String,
    pub visibility: Visibility,
    pub description: String,
    pub stats: VersionStats,
    pub author: String,
    pub review_state: ReviewState,
    pub channel: ProjectChannel,
    pub pinned_status: PinnedStatus,
    pub downloads: HashMap<Platform, PlatformVersionDownload>,
    pub plugin_dependencies: HashMap<Platform, Vec<PluginDependency>>,
    pub platform_dependencies: HashMap<Platform, Vec<String>>,
    pub platform_dependencies_formatted: HashMap<Platform, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PluginDependency {
    pub name: String,
    pub required: bool,
    pub external_url: Option<String>,
    pub platform: Platform,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Platform {
    Paper,
    Waterfall,
    Velocity,
}

impl From<&str> for Platform {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "waterfall" => Self::Waterfall,
            "velocity" => Self::Velocity,
            _ => Self::Paper,
        }
    }
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        match self {
            Self::Paper => "PAPER".to_owned(),
            Self::Waterfall => "WATERFALL".to_owned(),
            Self::Velocity => "VELOCITY".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum PlatformVersionDownload {
    #[serde(rename_all = "camelCase")]
    Hangar {
        file_info: FileInfo,
        download_url: String,
    },

    #[serde(rename_all = "camelCase")]
    External {
        file_info: FileInfo,
        external_url: String,
    },
}

impl PlatformVersionDownload {
    #[must_use]
    pub fn get_url(&self) -> String {
        match &self {
            Self::Hangar { download_url, .. } => download_url.clone(),
            Self::External { external_url, .. } => external_url.clone(),
        }
    }

    #[must_use]
    pub fn get_file_info(&self) -> FileInfo {
        match &self {
            Self::Hangar { file_info, .. } | Self::External { file_info, .. } => file_info.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    pub name: String,
    pub size_bytes: i64,
    pub sha256_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectChannel {
    pub created_at: String,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub flags: HashSet<ChannelFlag>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelFlag {
    Frozen,
    Unstable,
    Pinned,
    SendsNotifications,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum PinnedStatus {
    Version,
    Channel,
    #[default]
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReviewState {
    Unreviewed,
    Reviewed,
    UnderReview,
    PartiallyReviewed,
}

///

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionsFilter {
    pub limit: i64,
    pub offset: i64,
    pub channel: Option<String>,
    pub platform: Option<Platform>,
    pub platform_version: Option<String>,
}

impl Default for VersionsFilter {
    fn default() -> Self {
        Self {
            limit: 25,
            offset: 0,
            channel: None,
            platform: None,
            platform_version: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectVersionsResponse {
    pub pagination: Pagination,
    pub result: Vec<ProjectVersion>,
}

pub async fn fetch_project_versions(
    http_client: &reqwest::Client,
    id: &str,
    filter: Option<VersionsFilter>,
) -> Result<ProjectVersionsResponse> {
    let filter = filter.unwrap_or_default();

    Ok(http_client
        .get(format!(
            "{API_V1}/projects/{}/versions",
            if let Some((_, post)) = id.split_once('/') {
                post
            } else {
                id
            }
        ))
        .query(&filter)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

pub async fn fetch_project_version(
    http_client: &reqwest::Client,
    id: &str,
    name: &str,
) -> Result<ProjectVersion> {
    Ok(http_client
        .get(format!(
            "{API_V1}/projects/{}/versions/{name}",
            if let Some((_, post)) = id.split_once('/') {
                post
            } else {
                id
            }
        ))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?)
}

pub async fn fetch_latest_project_version(
    http_client: &reqwest::Client,
    id: &str,
    channel: &str,
) -> Result<String> {
    Ok(http_client
        .get(format!("{API_V1}/projects/{id}/latest"))
        .query(&[("channel", channel)])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}

pub async fn fetch_latest_project_release(
    http_client: &reqwest::Client,
    id: &str,
) -> Result<String> {
    Ok(http_client
        .get(format!("{API_V1}/projects/{id}/latestrelease"))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}

pub async fn download_project_version(
    http_client: &reqwest::Client,
    id: &str,
    name: &str,
    platform: &Platform,
) -> Result<reqwest::Response> {
    Ok(http_client
        .get(format!(
            "{API_V1}/projects/{id}/versions/{name}/{}/download",
            platform.to_string()
        ))
        .send()
        .await?
        .error_for_status()?)
}
