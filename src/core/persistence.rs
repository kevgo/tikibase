use crate::core::document::Document;
use crate::core::resource::Resource;
use crate::core::tikibase::Tikibase;
use rand::Rng;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Provides a Tikibase instance for the given directory.
pub fn load_base(dir: PathBuf) -> Tikibase {
    let mut docs = Vec::new();
    let mut resources = Vec::new();
    for entry in WalkDir::new(&dir) {
        let entry = entry.unwrap();
        let filename = entry.file_name().to_str().unwrap();
        if filename == "." || filename == "tikibase.json" {
            continue;
        }
        let path = entry.into_path();
        if is_md(path.extension()) {
            docs.push(Document::load(path));
        } else {
            resources.push(Resource { path });
        }
    }
    Tikibase {
        dir,
        docs,
        resources,
    }
}

pub fn load_file(filepath: &Path) -> String {
    let mut result = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end()
        .to_string();
    result.push('\n');
    result
}

pub fn save_file(filepath: &Path, content: &str) {
    let mut file = fs::File::create(&filepath).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

/// creates a Tikibase instance for testing in the './tmp' directory
pub fn tmpbase() -> Tikibase {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let rand: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(3)
        .map(char::from)
        .collect();
    let dir = std::path::PathBuf::from(format!("./tmp/{}-{}", timestamp, rand));
    match std::fs::create_dir_all(&dir) {
        Ok(_) => load_base(dir),
        Err(e) => panic!("{}", e),
    }
}

fn is_md(ext: Option<&std::ffi::OsStr>) -> bool {
    match ext {
        None => false,
        Some(ext) => ext.to_str().unwrap() == "md",
    }
}
