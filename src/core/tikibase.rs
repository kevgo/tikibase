use super::document;
use super::document::Document;
use walkdir::WalkDir;

pub struct Tikibase {
  pub dir: String,
  pub docs: Vec<Document>,
  pub resources: Vec<Resource>,
}

/// A non-Markdown file stored in a Tikibase.
pub struct Resource {
  pub path: std::path::PathBuf,
}

/// Provides a Tikibase instance for the given directory.
pub fn in_dir(dir: &str) -> Tikibase {
  let mut docs = Vec::new();
  let mut resources = Vec::new();
  for entry in WalkDir::new(dir) {
    let entry = entry.unwrap();
    let filename = entry.file_name().to_str().unwrap();
    if filename == "tikibase.json" || filename == "." {
      continue;
    }
    let path = entry.into_path();
    match path.extension() {
      None => resources.push(Resource { path }),
      Some(ext) => match ext.to_str().unwrap() {
        "md" => docs.push(document::load(path)),
        _ => resources.push(Resource { path }),
      },
    }
  }
  Tikibase {
    dir: dir.to_string(),
    docs,
    resources,
  }
}
