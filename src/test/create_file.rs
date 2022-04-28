use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

pub fn create_file(filename: &str, content: &str, workspace: &Path) {
    let file_path = workspace.join(filename);
    let parent_dir = file_path.parent().unwrap();
    fs::create_dir_all(parent_dir).unwrap();
    let mut file = File::create(file_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
