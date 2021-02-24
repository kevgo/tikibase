use std::path::PathBuf;

pub struct Document {
  path: PathBuf,
}

pub fn new(path: PathBuf) -> Document {
  let doc = Document { path };
  doc
}
