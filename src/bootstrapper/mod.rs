use anyhow::{bail, Context, Result};
use console::style;
use java_properties::read;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub struct BootstrapContext {
    pub output_dir: PathBuf,
    pub vars: HashMap<String, String>,
}

impl BootstrapContext {
    pub fn get_output_path(&self, path: &Path) -> PathBuf {
        let mut path_buf = PathBuf::new();
        let mut iter = path.components();

        if iter.next().is_some() {
            // Set first component as the output directory
            path_buf.push(self.output_dir.as_os_str());
        }

        // Append the remaining components to the new path
        path_buf.extend(iter);

        path_buf
    }
}

pub fn bootstrap<P>(ctx: &BootstrapContext, folder: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    // iterate over all files

    // create server directory if not exists
    if !Path::new(ctx.output_dir.as_path()).exists() {
        fs::create_dir(ctx.output_dir.as_path())?;
    }

    for entry in WalkDir::new(folder) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                bail!(
                    "Can't walk directory/file {}: {e}",
                    &e.path().unwrap_or(Path::new("unknown")).display()
                );
            }
        };

        if entry.file_type().is_dir() {
            continue;
        }

        bootstrap_entry(ctx, &entry)?;
    }

    Ok(())
}

fn bootstrap_entry(ctx: &BootstrapContext, entry: &DirEntry) -> Result<()> {
    let path = entry.path();
    let output_path = ctx.get_output_path(path);

    // bootstrap contents of some types
    if let Some(ext) = path.extension() {
        match ext.to_str().unwrap_or("") {
            "properties" => {
                let input = fs::read_to_string(path)?;

                let existing_input = String::from_utf8(fs::read(&output_path).unwrap_or(vec![]))?;
                let output = bootstrap_properties(ctx, &existing_input, &input)?;
                //probably
                fs::create_dir_all(output_path.parent().unwrap_or(Path::new("")))?;
                fs::write(output_path.clone(), output)
                    .context(format!("Writing {}", output_path.display()))?;
            }
            "txt" | "yaml" | "yml" | "conf" | "config" | "toml" => {
                let input = fs::read_to_string(path)?;
                let output = bootstrap_string(ctx, &input);
                fs::create_dir_all(output_path.parent().unwrap_or(Path::new("")))?;
                fs::write(output_path.clone(), output)
                    .context(format!("Writing {}", output_path.display()))?;
            }
            _ => {
                let output_path = ctx.get_output_path(path);
                fs::create_dir_all(output_path.clone())?;
                fs::copy(path, output_path.clone()).context(format!(
                    "Copying {} to {}",
                    path.display(),
                    output_path.display()
                ))?;
            }
        }
    } else {
        fs::create_dir_all(output_path.clone())?;
        fs::copy(path, output_path.clone()).context(format!(
            "Copying {} to {}",
            path.display(),
            output_path.display()
        ))?;
    }

    println!(
        "          {}",
        style("-> ".to_owned() + &output_path.display().to_string()).dim()
    );
    Ok(())
}

pub fn bootstrap_properties(
    ctx: &BootstrapContext,
    source_content: &str,
    existing_content: &str,
) -> Result<String> {
    let mut values = read(Cursor::new(source_content))?;
    values = values
        .iter()
        .map(|v| (v.0.clone(), bootstrap_string(ctx, v.1)))
        .collect();

    let mut existing_values: HashMap<String, String> = read(Cursor::new(existing_content))?;

    for (key, value) in values {
        existing_values.insert(key.clone(), value.clone());
    }

    let mut buffer: Vec<u8> = Vec::new();
    java_properties::write(&mut buffer, &existing_values)?;

    Ok(String::from_utf8(buffer)?)
}

pub fn bootstrap_string(ctx: &BootstrapContext, content: &str) -> String {
    if ctx.vars.contains_key("__NO_VARS") {
        return content.to_owned();
    }

    let re = Regex::new(r"\$\{(\w+)(?::([^}]+))?\}").unwrap();
    let replaced = re.replace_all(content, |caps: &regex::Captures| {
        let var_name = caps.get(2).map(|v| v.as_str()).unwrap_or_default();
        let default_value = caps.get(2).map(|v| v.as_str()).unwrap_or_default();

        if let Some(value) = ctx.vars.get(var_name) {
            value.to_string()
        } else {
            default_value.to_owned()
        }
    });
    replaced.into_owned()
}
