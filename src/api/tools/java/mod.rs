pub const JAVA_BIN: &str = "java";
pub type JavaVersion = u32;

mod installation;
mod find;
mod check;
pub use installation::*;
pub use find::*;
pub use check::*;

pub fn get_java_installations() -> Vec<JavaInstallation> {
    let paths = collect_possible_java_paths();

    paths.into_iter().filter_map(|path| check_java(&path)).collect()
}
