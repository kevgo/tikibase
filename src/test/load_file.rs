use super::trim_end;
use camino::Utf8Path;
use fs_err as fs;

// TODO: move this entire package into the test/cucumber directory
#[must_use]
pub fn load_file(filename: &str, dir: &Utf8Path) -> String {
  let mut result = match fs::read_to_string(dir.join(filename)) {
    Ok(text) => text,
    Err(err) => panic!("cannot open file \"{filename}\": {err}"),
  };
  trim_end(&mut result);
  result.push('\n');
  result
}
