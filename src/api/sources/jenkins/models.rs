use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JenkinsJobResponse {
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JenkinsBuildsResponse {
    pub builds: Vec<JenkinsBuild>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JenkinsArtifactsResponse {
    pub artifacts: Vec<JenkinsArtifact>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsBuild {
    pub url: String,
    pub number: i32,
    pub result: String,
    #[serde(default)]
    pub fingerprint: Vec<JenkinsFingerprint>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsArtifact {
    pub file_name: String,
    pub relative_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct JenkinsFingerprint {
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub hash: String,
}
