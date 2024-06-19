use serde::{Deserialize, Serialize};

use super::{Addon, AddonTarget, AddonType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonListFile {
    #[serde(default = "Vec::new")]
    pub addons: Vec<Addon>,
    
    // backwards compatability
    #[serde(default = "Vec::new")]
    pub mods: Vec<AddonType>,
    #[serde(default = "Vec::new")]
    pub plugins: Vec<AddonType>,
}

impl AddonListFile {
    pub fn flatten(self) -> Vec<Addon> {
        [
            self.addons,
            self.mods
                .into_iter()
                .map(|addon_type| {
                    Addon {
                        environment: None,
                        addon_type,
                        target: AddonTarget::Mod,
                    }
                })
                .collect(),
            self.plugins
                .into_iter()
                .map(|addon_type| {
                    Addon {
                        environment: None,
                        addon_type,
                        target: AddonTarget::Plugin,
                    }
                }).collect()
        ].concat()
    }
}
