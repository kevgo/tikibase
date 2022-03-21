use super::trim_end;
use std::fs;
use std::path::Path;

pub fn load_file<P: AsRef<Path>>(filename: P, dir: &Path) -> String {
    let mut result = fs::read_to_string(dir.join(filename)).unwrap();
    trim_end(&mut result);
    result.push('\n');
    result
}
