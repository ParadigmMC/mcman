use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

enum JavaLocation {
    Path(&'static str),
    PathSubpathAndWildcard(&'static str, Vec<&'static str>),
    PathWildcard(&'static str, Vec<&'static str>),
}

fn env_paths() -> HashSet<PathBuf> {
    let paths = env::var("PATH").map(|x| env::split_paths(&x).collect::<HashSet<_>>());
    paths.unwrap_or_else(|_| HashSet::new())
}

pub(super) fn collect_possible_java_paths() -> HashSet<PathBuf> {
    let locations = &get_possible_java_locations()[env::consts::OS];

    let mut paths = HashSet::new();

    paths.extend(env_paths());

    for loc in locations {
        match loc {
            JavaLocation::Path(path) => {
                paths.insert(path.into());
            },
            JavaLocation::PathWildcard(base, extras) => {
                if let Ok(entries) = PathBuf::from(base).read_dir() {
                    for entry in entries.flatten() {
                        for extra in extras {
                            paths.insert(entry.path().join(extra));
                        }
                    }
                }
            },
            JavaLocation::PathSubpathAndWildcard(base, extras) => {
                paths.insert(base.into());
                for extra in extras {
                    paths.insert(PathBuf::from(base).join(extra));
                }
                if let Ok(entries) = PathBuf::from(base).read_dir() {
                    for entry in entries.flatten() {
                        for extra in extras {
                            paths.insert(entry.path().join(extra));
                        }
                    }
                }
            },
        }
    }

    paths
}

fn get_possible_java_locations() -> HashMap<&'static str, Vec<JavaLocation>> {
    HashMap::from([
        (
            "windows",
            vec![
                JavaLocation::PathWildcard(r"C:/Program Files/Java", vec!["bin"]),
                JavaLocation::PathWildcard(r"C:/Program Files (x86)/Java", vec!["bin"]),
                JavaLocation::PathWildcard(r"C:\Program Files\Eclipse Adoptium", vec!["bin"]),
                JavaLocation::PathWildcard(r"C:\Program Files (x86)\Eclipse Adoptium", vec!["bin"]),
            ],
        ),
        (
            "macos",
            vec![
                JavaLocation::Path(
                    r"/Applications/Xcode.app/Contents/Applications/Application Loader.app/Contents/MacOS/itms/java",
                ),
                JavaLocation::Path(
                    r"/Library/Internet Plug-Ins/JavaAppletPlugin.plugin/Contents/Home",
                ),
                JavaLocation::Path(
                    r"/System/Library/Frameworks/JavaVM.framework/Versions/Current/Commands",
                ),
                JavaLocation::PathWildcard(
                    r"/Library/Java/JavaVirtualMachines/",
                    vec![r"Contents/Home/bin"],
                ),
            ],
        ),
        (
            "linux",
            vec![
                JavaLocation::PathSubpathAndWildcard(r"/usr", vec!["jre/bin", "bin"]),
                JavaLocation::PathSubpathAndWildcard(r"/usr/java", vec!["jre/bin", "bin"]),
                JavaLocation::PathSubpathAndWildcard(r"/usr/lib/jvm", vec!["jre/bin", "bin"]),
                JavaLocation::PathSubpathAndWildcard(r"/usr/lib64/jvm", vec!["jre/bin", "bin"]),
                JavaLocation::PathSubpathAndWildcard(r"/opt/jdk", vec!["jre/bin", "bin"]),
                JavaLocation::PathSubpathAndWildcard(r"/opt/jdks", vec!["jre/bin", "bin"]),
            ],
        ),
    ])
}
