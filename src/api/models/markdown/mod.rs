use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MarkdownOptions {
    pub files: Vec<String>,
    pub columns: Vec<MdColumn>,
    pub sort_by: MdSort,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            files: vec![String::from("README.md")],
            columns: vec![
                MdColumn::Icon,
                MdColumn::Name,
                MdColumn::Description,
                MdColumn::Version,
            ],
            sort_by: MdSort::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MdColumn {
    Icon,
    Name,
    Description,
    Version,
}


#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MdSort {
    Alphabetical,
    Source,
    #[default]
    None,
}

