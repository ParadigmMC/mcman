use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use super::{
    ArgumentValue, PistonArgument, PistonFile, PistonLibrary, PistonOs, PistonRule,
    PistonRuleConstraints,
};

/// `PistonRuleMatcher` is an utility for matching argument and library rules
pub struct PistonRuleMatcher {
    pub os: PistonOs,
    pub features: HashMap<String, bool>,
}

impl PistonRuleMatcher {
    /// Creates a piston rule matcher with the provided OS information
    #[must_use]
    pub fn new(os_name: String, os_arch: String, os_version: String) -> Self {
        Self {
            os: PistonOs {
                name: os_name,
                arch: os_arch,
                version: os_version,
            },
            features: HashMap::new(),
        }
    }

    /// Creates a piston rule matcher with empty OS information
    #[must_use]
    pub fn empty() -> Self {
        Self {
            os: PistonOs {
                name: String::new(),
                arch: String::new(),
                version: String::new(),
            },
            features: HashMap::new(),
        }
    }

    /// Creates a rule matcher with OS info from the current system.
    /// Use `PistonRuleMatcher::new()` if OS detection is not desired.
    #[must_use]
    pub fn from_os() -> Self {
        let info = os_info::get();
        let os_name = match info.os_type() {
            os_info::Type::Windows => "windows",
            os_info::Type::Macos => "osx",
            _ => "linux", // Close enough
        }
        .to_owned();

        let os_version = match info.version() {
            os_info::Version::Unknown => String::new(),
            v => v.to_string(),
        };

        Self {
            os: PistonOs {
                name: os_name,
                arch: std::env::consts::ARCH.to_owned(),
                version: os_version,
            },
            features: HashMap::new(),
        }
    }

    pub fn should_download_library(&self, library: &PistonLibrary) -> bool {
        self.match_rules(&library.rules)
    }

    /// find classifier from library.
    /// Some(PistonFile) if classifier for matcher exists
    /// None if no classifiers exist/no matches
    #[must_use]
    pub fn get_native_library(&self, library: &PistonLibrary) -> Option<PistonFile> {
        if let Some(native_keys) = &library.natives {
            if let Some(classifier_key) = native_keys.get(&self.os.name) {
                if let Some(map) = &library.downloads.classifiers {
                    return Some(
                        map[&self.process_string(&HashMap::new(), classifier_key)].clone(),
                    );
                }
            }
        }

        None
    }

    pub fn match_rules(&self, rules: &Vec<PistonRule>) -> bool {
        if rules.is_empty() {
            return true;
        }

        for rule in rules {
            if !self.match_rule(rule) {
                return false;
            }
        }

        true
    }

    pub fn match_rule(&self, rule: &PistonRule) -> bool {
        match rule {
            PistonRule::Allow(constraint) => self.match_constraint(constraint),
            PistonRule::Disallow(constraint) => {
                // Fuck it
                !self.match_constraint(constraint)
            },
        }
    }

    pub fn match_constraint(&self, constraint: &PistonRuleConstraints) -> bool {
        if let Some(os) = &constraint.os {
            if !os.name.is_empty() && os.name != self.os.name {
                return false;
            }

            if !os.arch.is_empty() && os.arch != self.os.arch {
                return false;
            }

            if !os.version.is_empty()
                && !Regex::new(&os.version).unwrap().is_match(&self.os.version)
            {
                return false;
            }
        }

        if let Some(feats) = &constraint.features {
            for feat in feats.keys() {
                if !self.features.contains_key(feat) || !self.features[feat] {
                    return false;
                }
            }
        }

        true
    }

    pub fn build_args(
        &self,
        args: &[PistonArgument],
        map: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut list: Vec<String> = vec![];
        for arg in args {
            match arg {
                PistonArgument::Normal(str) => list.push(str.to_owned()),
                PistonArgument::Ruled { rules, value } => {
                    if self.match_rules(rules) {
                        match value {
                            ArgumentValue::Single(v) => list.push(v.to_owned()),
                            // bad
                            ArgumentValue::Many(li) => {
                                li.iter().for_each(|v| list.push(v.to_owned()));
                            },
                        };
                    }
                },
            }
        }

        list.iter().map(|s| self.process_string(map, s)).collect()
    }

    #[must_use]
    pub fn process_string(&self, map: &HashMap<String, String>, input: &str) -> String {
        dollar_repl(input, |key| {
            if key == "arch" {
                return Some(self.os.arch.clone());
            }

            map.get(key).cloned()
        })
    }
}

lazy_static! {
    static ref DOLLAR_REGEX: Regex = Regex::new(r"\$\{(\w+)?\}").unwrap();
}

fn dollar_repl<F>(input: &str, replacer: F) -> String
where
    F: Fn(&str) -> Option<String>,
{
    DOLLAR_REGEX
        .replace_all(input, |caps: &regex::Captures| {
            let var_name = caps.get(1).map(|v| v.as_str()).unwrap_or_default();

            if let Some(v) = replacer(var_name) {
                v
            } else {
                format!("${{{var_name}}}")
            }
        })
        .into_owned()
}
