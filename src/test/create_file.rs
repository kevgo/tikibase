use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn create_file(filename: &str, content: &str, dir: &Path) {
    let mut file = File::create(dir.join(filename)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
