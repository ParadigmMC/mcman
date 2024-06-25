pub const JAVA_BIN: &str = "java";
pub type JavaVersion = u32;

mod installation;
mod find;
mod check;
use futures::StreamExt;
pub use installation::*;
pub use check::*;

pub struct JavaProcess {
    
}

pub async fn get_java_installations() -> Vec<JavaInstallation> {
    let paths = find::collect_possible_java_paths();

    futures::stream::iter(paths)
        .filter_map(|path| async move {
            check_java(&path)
        })
        .collect()
        .await
}
