use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogFileMetadata {
    pub id: String,
    pub url: String,
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogInsights {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub log_type: String,
    pub version: String,
    pub title: String,
    pub analysis: LogAnalysis,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LogAnalysis {
    pub problems: Vec<Problem>,
    pub information: Vec<Information>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Problem {
    pub message: String,
    pub counter: usize,
    pub entry: AnalysisEntry,
    pub solutions: Vec<Solution>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Information {
    pub message: String,
    pub counter: usize,
    pub label: String,
    pub value: String,
    pub entry: AnalysisEntry,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnalysisEntry {
    pub level: usize,
    pub time: Option<String>,
    pub prefix: String,
    pub lines: Vec<AnalysisLine>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AnalysisLine {
    pub number: usize,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Solution {
    pub message: String,
}
