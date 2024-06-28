use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod render;

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq)]
pub enum MarkdownOutput {
    #[default]
    ASCII,
    HTML,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MarkdownOptions {
    pub files: Vec<String>,
    pub columns: Vec<MdColumn>,
    pub titles: HashMap<MdColumn, String>,
    pub name_includes_link: bool,
    pub sort_by: MdSort,
    pub output_type: MarkdownOutput,
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
            name_includes_link: true,
            titles: HashMap::new(),
            sort_by: MdSort::default(),
            output_type: MarkdownOutput::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum MdColumn {
    Icon,
    Name,
    Description,
    Version,
    Link,
}

impl ToString for MdColumn {
    fn to_string(&self) -> String {
        match self {
            MdColumn::Icon => String::from("."),
            MdColumn::Name => String::from("Name"),
            MdColumn::Description => String::from("Description"),
            MdColumn::Version => String::from("Version"),
            MdColumn::Link => String::from("Link"),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MdSort {
    Alphabetical,
    Source,
    #[default]
    None,
}

