use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::api::{
    models::Environment,
    utils::serde::{SingleOrArray, SingleOrHashMap},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Person {
    String(String),
    Object {
        name: String,
        contact: HashMap<String, String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NestedJarEntry {
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MixinEntry {
    String(String),
    Object {
        config: String,
        // ???
    },
}

pub type VersionRange = SingleOrArray<String>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FabricModJson {
    pub id: String,
    pub version: String,

    pub name: Option<String>,
    pub description: Option<String>,
    pub authors: Vec<Person>,
    pub contributors: Vec<Person>,
    pub contact: HashMap<String, String>,

    pub license: SingleOrArray<String>,
    pub icon: SingleOrHashMap<String, String>,

    pub environment: Option<SingleOrArray<Environment>>,
    pub entrypoints: (),
    pub jars: Vec<NestedJarEntry>,
    pub language_adapters: HashMap<String, String>,
    pub mixins: (),
    pub access_widener: Option<String>,

    pub depends: HashMap<String, VersionRange>,
    pub recommends: HashMap<String, VersionRange>,
    pub suggests: HashMap<String, VersionRange>,
    pub conflicts: HashMap<String, VersionRange>,
    pub breaks: HashMap<String, VersionRange>,

    pub custom: HashMap<String, serde_json::Value>,
}
