use fs_err as fs;
use fs_err::File;
use std::io::prelude::*;
use std::path::Path;

pub fn create_file(filename: impl AsRef<Path>, content: &str, dir: impl AsRef<Path>) {
  let filename = filename.as_ref();
  let dir = dir.as_ref();
  if let Some(parent) = filename.parent() {
    fs::create_dir_all(dir.join(parent)).unwrap();
  }
  let mut file = File::create(dir.join(filename)).unwrap();
  file.write_all(content.as_bytes()).unwrap();
}
