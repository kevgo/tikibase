use crate::core::config;
use crate::core::line::Line;
use crate::core::section::Section;
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

pub fn create_file(filename: &str, content: &str, dir: &Path) {
    let mut file = File::create(dir.join(filename)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

/// provides an empty configuration instance for testing
pub fn empty_config() -> config::Data {
    config::Data {
        sections: None,
        ignore: None,
    }
}

/// provides a Line with the given text
pub fn line_with_text(text: &str) -> Line {
    Line { text: text.into() }
}

pub fn load_file<P: AsRef<Path>>(filename: P, dir: &Path) -> String {
    let mut result = std::fs::read_to_string(dir.join(filename))
        .unwrap()
        .trim_end()
        .to_string();
    result.push('\n');
    result
}

/// provides a section with the given title for testing
pub fn section_with_title(title: &str) -> Section {
    Section {
        line_number: 0,
        title_line: Line { text: title.into() },
        body: vec![],
    }
}
