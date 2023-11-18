use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use pathdiff::diff_paths;

pub fn try_get_url(folder: &PathBuf) -> Result<String> {
    let repo_url = get_git_remote()?.ok_or(anyhow!("cant get repo url"))?;
    let root = get_git_root()?.ok_or(anyhow!("cant get repo root"))?;
    let branch = get_git_branch()?.ok_or(anyhow!("cant get repo branch"))?;

    let diff = diff_paths(folder, root).ok_or(anyhow!("cant diff paths"))?;

    let repo = if repo_url.starts_with("https") {
        repo_url.strip_prefix("https://github.com/")
    } else {
        repo_url.strip_prefix("http://github.com/")
    }
    .ok_or(anyhow!("repo not on github?"))?;

    let parent_paths = diff.to_string_lossy().replace('\\', "/");
    let parent_paths = if parent_paths.is_empty() {
        parent_paths
    } else {
        "/".to_owned() + &parent_paths
    };

    Ok(repo.to_owned() + "/" + &branch + &parent_paths)
}

pub fn get_git_remote() -> Result<Option<String>> {
    let path = git_command(vec!["remote", "get-url", "origin"])?
        .ok_or(anyhow!("cant get git repo origin"))?;

    Ok(Some(
        path.strip_suffix(".git")
            .map_or(path.clone(), ToOwned::to_owned),
    ))
}

pub fn write_git() -> Result<()> {
    write_gitignore()?;
    write_gitattributes()?;
    Ok(())
}

pub fn write_gitignore() -> Result<PathBuf> {
    let root = get_git_root()?.ok_or(anyhow!("cant get repo root"))?;

    let gitignore_path = Path::new(&root).join(".gitignore");

    let contents = fs::read_to_string(&gitignore_path).unwrap_or_default();

    let has_r = contents.contains('\r');

    let contents = contents.replace('\r', "");

    let mut list = contents.split('\n').collect::<Vec<_>>();

    for (ignore, comment) in [
        ("**/server", "# mcman: Exclude mcman build outputs"),
        ("*.mrpack", "# mcman: Exclude exported mrpacks"),
        ("**/.env", "# mcman: Exclude local dotenv files"),
    ] {
        if !list.contains(&ignore) {
            if !comment.is_empty() {
                list.push(comment);
            }
            list.push(ignore);
        }
    }

    let contents = list.join(if has_r { "\r\n" } else { "\n" });

    fs::write(&gitignore_path, contents)?;

    Ok(gitignore_path)
}

pub fn write_gitattributes() -> Result<PathBuf> {
    let root = get_git_root()?.ok_or(anyhow!("cant get repo root"))?;

    let gitattributes_path = Path::new(&root).join(".gitattibutes");

    let contents = fs::read_to_string(&gitattributes_path).unwrap_or_default();

    let has_r = contents.contains('\r');

    let contents = contents.replace('\r', "");

    let mut list = contents.split('\n').collect::<Vec<_>>();

    for (attr, comment) in [
        ("**/worlds/*.zip filter=lfs diff=lfs merge=lfs -text", "# mcman: use lfs for worlds"),
    ] {
        if !list.contains(&attr) {
            if !comment.is_empty() {
                list.push(comment);
            }
            list.push(attr);
        }
    }

    let contents = list.join(if has_r { "\r\n" } else { "\n" });

    fs::write(&gitattributes_path, contents)?;

    Ok(gitattributes_path)
}

pub fn get_git_root() -> Result<Option<String>> {
    git_command(vec!["rev-parse", "--show-toplevel"])
}

pub fn get_git_branch() -> Result<Option<String>> {
    git_command(vec!["rev-parse", "--abbrev-ref", "HEAD"])
}

pub fn git_command(args: Vec<&str>) -> Result<Option<String>> {
    run_command("git", args)
}

pub fn write_dockerfile(folder: &Path) -> Result<()> {
    let mut f = File::create(folder.join("Dockerfile"))?;
    f.write_all(include_bytes!("../../res/default_dockerfile"))?;
    Ok(())
}

pub fn write_dockerignore(folder: &Path) -> Result<()> {
    let mut f = File::create(folder.join(".dockerignore"))?;
    f.write_all(include_bytes!("../../res/default_dockerignore"))?;
    Ok(())
}

pub fn get_docker_version() -> Result<Option<String>> {
    run_command("docker", vec!["--version"])
}

pub fn run_command(prog: &str, args: Vec<&str>) -> Result<Option<String>> {
    let output = std::process::Command::new(prog).args(args).output()?;

    Ok(if output.status.success() {
        let path = String::from_utf8_lossy(output.stdout.as_slice())
            .into_owned()
            .trim()
            .to_owned();
        Some(path)
    } else {
        None
    })
}
