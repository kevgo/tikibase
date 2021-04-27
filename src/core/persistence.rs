use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn load_file(filepath: &Path) -> String {
    let mut result = std::fs::read_to_string(filepath)
        .unwrap()
        .trim_end()
        .to_string();
    result.push('\n');
    result
}

pub fn create_file(filepath: &Path, content: &str) {
    let mut file = fs::File::create(&filepath).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
