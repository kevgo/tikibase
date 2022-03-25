use super::trim_end;
use std::fs;
use std::path::Path;

pub fn load_file<P: AsRef<Path>>(filename: P, dir: &Path) -> String {
    let mut result = match fs::read_to_string(dir.join(filename.as_ref())) {
        Ok(text) => text,
        Err(err) => panic!(
            "cannot open file \"{}\": {}",
            filename.as_ref().to_string_lossy(),
            err
        ),
    };
    trim_end(&mut result);
    result.push('\n');
    result
}
