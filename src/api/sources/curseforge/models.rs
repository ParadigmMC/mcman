use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeMod {
    pub id: i32,
    pub game_id: i32,
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub download_count: i64,
    pub is_featured: bool,
    pub allow_mod_distribution: bool,
    pub latest_files: Vec<CurseforgeFile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeFile {
    pub id: i32,
    pub game_id: i32,
    pub mod_id: i32,
    pub is_available: bool,
    pub display_name: String,
    pub release_type: FileReleaseType,
    pub hashes: Vec<CurseforgeFileHash>,
    pub file_length: u64,
    pub download_url: String,
    pub game_versions: Vec<String>,
    pub dependencies: Vec<CurseforgeDependency>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CurseforgeDependency {
    pub mod_id: i32,
    pub relation_type: CurseforgeDependencyType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CurseforgeDependencyType {
    EmbeddedLibrary = 1,
    OptionalDependency = 2,
    RequiredDependency = 3,
    Tool = 4,
    Incompatible = 5,
    Include = 6,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurseforgeFileHash {
    pub value: String,
    pub algo: CurseforgeHashAlgo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CurseforgeHashAlgo {
    Sha1 = 1,
    Md5 = 2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FileReleaseType {
    Release = 1,
    Beta = 2,
    Alpha = 3,
}
