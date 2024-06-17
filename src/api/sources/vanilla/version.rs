use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::VersionType;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub id: String,
    pub assets: String,
    pub asset_index: PistonFile,
    pub java_version: VersionJavaInfo,
    pub libraries: Vec<PistonLibrary>,

    pub downloads: HashMap<DownloadType, PistonFile>,

    pub arguments: VersionArguments,
    pub minecraft_arguments: String,

    pub compliance_level: u8,
    pub minimum_launcher_version: u8,

    pub main_class: String,
    pub logging: LoggingInfoWrapper,

    #[serde(rename = "type")]
    pub version_type: VersionType,
    pub time: String,
    pub release_time: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DownloadType {
    Client,
    ClientMappings,
    Server,
    ServerMappings,
    WindowsServer,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct VersionJavaInfo {
    pub major_version: u8,
    pub component: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct VersionArguments {
    pub game: Vec<PistonArgument>,
    pub jvm: Vec<PistonArgument>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum PistonArgument {
    Normal(String),
    Ruled {
        rules: Vec<PistonRule>,
        value: ArgumentValue,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ArgumentValue {
    Single(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct PistonLibrary {
    pub name: String,
    pub downloads: PistonLibraryDownload,
    pub rules: Vec<PistonRule>,

    /// Present on old versions, something like this:
    /// "extract": {
    ///     "exclude": ["META-INF/"],
    ///     "name": "tv.twitch:twitch-external-platform:4.5"
    /// }
    pub extract: Option<PistonExtractLibrary>,

    /// Present on old versions, some weird stuff involving classifiers
    /// "natives": {
    ///     "linux":   "natives-linux"
    ///     "osx":     "natives-osx"
    ///     "windows": "natives-windows-${arch}"
    /// }
    pub natives: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PistonExtractLibrary {
    exclude: Vec<String>,
    name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "action")]
pub enum PistonRule {
    Allow(PistonRuleConstraints),
    Disallow(PistonRuleConstraints),
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PistonRuleConstraints {
    pub os: Option<PistonOs>,
    pub features: Option<HashMap<String, bool>>,
}

/*
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PistonRuleConstraintFeature {
    pub is_demo_user: bool,
    pub has_custom_resolution: bool,
    pub has_quick_plays_support: bool,

} */

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PistonOs {
    pub name: String,
    pub arch: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct PistonLibraryDownload {
    pub artifact: PistonFile,

    /// Conditional files that may be needed to be downloaded alongside the library
    /// The HashMap key specifies a classifier as additional information for downloading files
    pub classifiers: Option<HashMap<String, PistonFile>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoggingInfoWrapper {
    pub client: VersionLoggingInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VersionLoggingInfo {
    pub argument: String,
    pub file: PistonFile,
    #[serde(rename = "type")]
    pub logging_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct PistonFile {
    pub sha1: String,
    /// Size of file at url
    pub size: u64,
    pub url: String,

    /// (AssetIndex only) The game version ID the assets are for
    pub id: Option<String>,
    /// (AssetIndex only) The size of the game version's assets
    pub total_size: Option<u64>,

    /// Only present on library files
    pub path: Option<String>,
}
