use serde::{Deserialize, Serialize};

mod addon_metadata;

pub use addon_metadata::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataContainer {
    pub addons: Vec<AddonMetadata>,
}
