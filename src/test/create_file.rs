use crate::database::paths;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

pub fn create_file(filename: &str, content: &str, workspace: &str) {
    let file_path = paths::join(workspace, filename);
    let file_path = PathBuf::from(file_path);
    let parent_dir = file_path.parent().unwrap();
    fs::create_dir_all(parent_dir).unwrap();
    let mut file = File::create(file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
