use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// creates a temporary directory
pub fn tmp_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let rand: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(3)
        .map(char::from)
        .collect();
    let dir = PathBuf::from(format!("./tmp/{}-{}", timestamp, rand));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[allow(dead_code)]
pub fn create_file(filename: &str, content: &str, dir: &Path) {
    let mut file = File::create(dir.join(filename)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}
