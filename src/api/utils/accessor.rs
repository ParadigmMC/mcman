use std::path::PathBuf;

pub enum Accessor {
    Local(PathBuf),
    Remote(reqwest::Url),
}

impl Accessor {
    
}
