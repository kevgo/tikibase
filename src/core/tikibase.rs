use super::document;
use super::document::Document;
use walkdir::WalkDir;

pub struct Tikibase {
  pub dir: String,
  pub docs: Vec<Document>,
}

/// Provides a Tikibase instance for the given directory.
pub fn in_dir(dir: &str) -> Tikibase {
  let mut docs = Vec::new();
  for entry in WalkDir::new(dir) {
    docs.push(document::new(entry.unwrap().into_path()));
  }
  Tikibase {
    dir: dir.to_string(),
    docs,
  }
}
