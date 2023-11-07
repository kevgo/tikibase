use super::trim_end;
use crate::database::paths;
use fs_err as fs;

pub fn load_file(filename: &str, dir: &str) -> String {
    let mut result = match fs::read_to_string(paths::join(dir, filename)) {
        Ok(text) => text,
        Err(err) => panic!("cannot open file \"{filename}\": {err}"),
    };
    trim_end(&mut result);
    result.push('\n');
    result
}
