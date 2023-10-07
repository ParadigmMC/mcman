use anyhow::Result;
use regex::Regex;

pub mod env;
pub mod hash;
pub mod maven_import;
pub mod md;
pub mod packwiz;
pub mod logger;

pub struct SelectItem<T>(pub T, pub String);

impl<T> ToString for SelectItem<T> {
    fn to_string(&self) -> String {
        self.1.clone()
    }
}

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

pub fn is_default_str(s: &str) -> bool {
    s == "latest"
}

pub fn get_latest_semver(list: &[String]) -> Option<String> {
    let mut list = list
        .iter()
        .filter_map(|s| semver::Version::parse(s).ok())
        .collect::<Vec<_>>();

    list.sort_by(semver::Version::cmp);

    list.last().map(ToString::to_string)
}

/// ci.luckto.me => ci-lucko-me
pub fn url_to_folder(url: &str) -> String {
    let folder = url.replace("https://", "");
    let folder = folder.replace("http://", "");
    let folder = folder.replace("/", " ");
    let folder = folder.trim();
    let folder = folder.replace(" ", "-");
    folder
}

static SANITIZE_R1: &str = "<(?:\"[^\"]*\"['\"]*|'[^']*'['\"]*|[^'\">])+>";

pub fn sanitize(s: &str) -> Result<String> {
    let re = Regex::new(SANITIZE_R1)?;

    Ok(re
        .replace_all(
            &s.replace('\n', " ").replace('\r', "").replace("<br>", " "),
            "",
        )
        .to_string())
}
