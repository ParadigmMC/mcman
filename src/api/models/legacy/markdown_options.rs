use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(default)]
pub struct LegacyMarkdownOptions {
    pub files: Vec<String>,
    pub auto_update: bool,
}

impl Default for LegacyMarkdownOptions {
    fn default() -> Self {
        Self {
            files: vec!["README.md".to_owned()],
            auto_update: false,
        }
    }
}

impl LegacyMarkdownOptions {
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}
