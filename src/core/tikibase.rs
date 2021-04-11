use super::document::Document;
use walkdir::WalkDir;

pub struct Tikibase {
    pub dir: String,
    pub docs: Vec<Document>,
    pub resources: Vec<Resource>,
}

impl Tikibase {
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
            if is_md(path.extension()) {
                docs.push(Document::load(path));
            } else {
                resources.push(Resource { path });
            }
        }
        Tikibase {
            dir: dir.to_string(),
            docs,
            resources,
        }
    }

    #[allow(dead_code)] // used in tests
    pub fn with_doc(doc: Document) -> Tikibase {
        Tikibase {
            dir: "".to_string(),
            docs: vec![doc],
            resources: vec![],
        }
    }
}

/// A non-Markdown file stored in a Tikibase.
pub struct Resource {
    pub path: std::path::PathBuf,
}

fn is_md(ext: Option<&std::ffi::OsStr>) -> bool {
    match ext {
        None => false,
        Some(ext) => ext.to_str().unwrap() == "md",
    }
}
