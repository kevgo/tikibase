use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn create_file<P: AsRef<Path>>(filename: &str, content: &str, dir: P) {
    let mut file = File::create(dir.as_ref().join(filename)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
