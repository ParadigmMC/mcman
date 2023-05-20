use std::collections::HashMap;
use std::fs;
use std::io::{Cursor};
use std::path::{Path, PathBuf};
use java_properties::read;
use regex::Regex;
use walkdir::{WalkDir, DirEntry};

use crate::error::Result;

pub struct BootstrapContext {
    pub output_dir: PathBuf,
    pub vars: HashMap<String, String>,
}

impl BootstrapContext {
    pub fn get_output_path(&self, path: &Path) -> PathBuf {
        let mut path_buf = PathBuf::new();
        let mut iter = path.components();
    
        if let Some(_) = iter.next() {
            // Set first component as the output directory    
            path_buf.push(self.output_dir.as_os_str());
        }
    
        // Append the remaining components to the new path
        path_buf.extend(iter);
    
        path_buf
    }
}

pub fn bootstrap(
    ctx: &BootstrapContext,
) {
    // iterate over all files

    // create server directory if not exists
    if !Path::new("server").exists() {
        fs::create_dir("server").unwrap();
    }
    
    for entry in WalkDir::new("config") {
        if !entry.is_ok() {
            println!("Error occurred while bootstrapping: Can't walk directory/file {} (check permissions?)",
                &entry.unwrap_err().path().unwrap_or(&Path::new("unknown")).display());
            continue;
        }

        let entry = entry.unwrap();

        if entry.file_type().is_dir() {
            continue;
        }

        match bootstrap_entry(&ctx, &entry) {
            Ok(_) => {},
            Err(e) => {
                // todo: handle errors better, with cooler output styles etc
                println!("Error occurred while bootstrapping: {:#?}", e);
            }
        }
    }
}

fn bootstrap_entry(
    ctx: &BootstrapContext,
    entry: &DirEntry,
) -> Result<()> {
    let path = entry.path();

    // bootstrap contents of some types
    if let Some(ext) = path.extension() {
        match ext.to_str().unwrap_or("") {
            "properties" => {
                let input = fs::read_to_string(path)?;
                let output_path = ctx.get_output_path(path);
                let existing_input = String::from_utf8(fs::read(&output_path).unwrap_or(vec![]))?;
                let output = bootstrap_properties(ctx, &existing_input, &input)?;
                fs::write(output_path, output).unwrap();
            },
            _ => {
                fs::copy(path, ctx.get_output_path(path)).unwrap();
            },
        }
    } else {
        let new_path = ctx.get_output_path(path);
        fs::copy(path, new_path).unwrap();
    }

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
        .map(|v| (v.0.to_owned(), bootstrap_string(ctx, &v.1)))
        .collect();

    let mut existing_values: HashMap<String, String> = read(Cursor::new(existing_content))?;
    
    for (key, value) in values {
        existing_values.insert(key.clone(), value.clone());
    }

    let mut buffer: Vec<u8> = Vec::new();
    java_properties::write(&mut buffer, &existing_values)?;

    Ok(String::from_utf8(buffer)?)
}

pub fn bootstrap_string(
    ctx: &BootstrapContext,
    content: &str,
) -> String {
    let re = Regex::new(r"\$\{(\w+)\}").unwrap();
    let replaced = re.replace_all(content, |caps: &regex::Captures| {
        if let Some(value) = ctx.vars.get(&caps[1]) {
            value.to_string()
        } else {
            caps[0].to_string()
        }
    });
    replaced.into_owned()
}

