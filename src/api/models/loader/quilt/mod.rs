use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{
    api::{
        models::Environment,
        utils::serde::{str_default, string_or_struct, SingleOrArray},
    },
    generate_de_vec,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuiltModJson {
    pub quilt_loader: QuiltModJsonLoader,
    pub minecraft: Option<QuiltModJsonMinecraft>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuiltEntrypoint {
    #[serde(default = "str_default")]
    pub adapter: String,
    pub value: String,
}

impl FromStr for QuiltEntrypoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            adapter: str_default(),
            value: s.to_owned(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuiltDependency {
    pub id: String,
    pub versions: Option<serde_json::Value>,
    pub reason: Option<String>,
    pub optional: bool,
    pub unless: Option<Box<QuiltDependency>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuiltProvides {
    pub id: String,
    pub version: Option<String>,
}

generate_de_vec!(QuiltProvides, de_vec_quilt_provides, "string_or_struct");
generate_de_vec!(QuiltEntrypoint, de_vec_quilt_entrypoint, "string_or_struct");

impl FromStr for QuiltProvides {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            id: s.to_owned(),
            version: None,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuiltModJsonLoader {
    pub group: String,
    pub id: String,
    pub version: String,
    #[serde(deserialize_with = "de_vec_quilt_provides")]
    #[serde(default = "Vec::new")]
    pub provides: Vec<QuiltProvides>,
    #[serde(default = "HashMap::new")]
    pub entrypoints: HashMap<String, SingleOrArray<QuiltEntrypoint>>,
    #[serde(deserialize_with = "de_vec_quilt_entrypoint")]
    #[serde(default = "Vec::new")]
    pub plugins: Vec<QuiltEntrypoint>,
    #[serde(default = "Vec::new")]
    pub jars: Vec<String>,
    #[serde(default = "HashMap::new")]
    pub language_adapters: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuiltModJsonMinecraft {
    pub environment: Environment,
}
