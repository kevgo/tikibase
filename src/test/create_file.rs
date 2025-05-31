use camino::Utf8Path;
use fs_err as fs;
use fs_err::File;
use std::io::prelude::*;

pub fn create_file<P1: AsRef<Utf8Path>, P2: AsRef<Utf8Path>>(filename: P1, content: &str, dir: P2) {
  let filename = filename.as_ref();
  let dir = dir.as_ref();
  if let Some(parent) = filename.parent() {
    fs::create_dir_all(dir.join(parent)).unwrap();
  }
  let mut file = File::create(dir.join(filename)).unwrap();
  file.write_all(content.as_bytes()).unwrap();
}
