use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::api::utils::serde::*;

pub mod render;

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, JsonSchema)]
pub enum MarkdownOutput {
    #[default]
    ASCII,
    HTML,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct MarkdownOptions {
    pub files: Vec<String>,
    #[serde(skip_serializing_if = "is_default", default = "default_columns")]
    pub columns: Vec<MdColumn>,
    #[serde(skip_serializing_if = "is_default", default)]
    pub titles: HashMap<MdColumn, String>,
    #[serde(skip_serializing_if = "is_true", default = "bool_true")]
    pub name_includes_link: bool,
    #[serde(skip_serializing_if = "is_default", default)]
    pub sort_by: MdSort,
    #[serde(skip_serializing_if = "is_default", default)]
    pub output_type: MarkdownOutput,
}

fn default_columns() -> Vec<MdColumn> {
    vec![
        MdColumn::Icon,
        MdColumn::Name,
        MdColumn::Description,
        MdColumn::Version,
    ]
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            files: vec![String::from("README.md")],
            columns: default_columns(),
            name_includes_link: true,
            titles: HashMap::new(),
            sort_by: MdSort::default(),
            output_type: MarkdownOutput::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, JsonSchema)]
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


#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum MdSort {
    Alphabetical,
    Source,
    #[default]
    None,
}

