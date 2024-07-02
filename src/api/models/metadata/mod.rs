mod addon_metadata;

pub use addon_metadata::*;

pub enum MetadataBlock {
    Addons(Vec<AddonMetadata>),
}
