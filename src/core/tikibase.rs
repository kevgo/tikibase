use super::document::Document;
use std::io::prelude::*;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Tikibase {
    pub dir: PathBuf,
    pub docs: Vec<Document>,
    pub resources: Vec<Resource>,
}

impl Tikibase {
    pub fn create_doc(&mut self, filename: &str, content: &str) {
        let filepath = self.dir.join(filename);
        let mut file = std::fs::File::create(&filepath).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        let doc = Document::from_str(content, filepath);
        self.docs.push(doc);
    }

    /// Provides a Tikibase instance for the given directory.
    pub fn in_dir(dir: PathBuf) -> Tikibase {
        let mut docs = Vec::new();
        let mut resources = Vec::new();
        for entry in WalkDir::new(&dir) {
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
            dir,
            docs,
            resources,
        }
    }

    #[allow(dead_code)] // used in tests
    pub fn with_doc(doc: Document) -> Tikibase {
        Tikibase {
            dir: PathBuf::from(""),
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

// ----------------------------------------------------------------------------------------------------------------------
// HELPERS
// ----------------------------------------------------------------------------------------------------------------------

pub mod helpers {
    use super::Tikibase;
    use rand::Rng;

    /// creates a Tikibase instance for testing
    pub fn testbase() -> Tikibase {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let rand: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(3)
            .map(char::from)
            .collect();
        let dir = std::path::PathBuf::from(format!("./tmp/{}-{}", timestamp, rand));
        match std::fs::create_dir_all(&dir) {
            Ok(_) => Tikibase::in_dir(dir),
            Err(e) => panic!("{}", e),
        }
    }

    pub fn read_doc(base: &Tikibase, filename: &str) -> String {
        let filepath = base.dir.join(filename);
        let mut result = std::fs::read_to_string(filepath)
            .unwrap()
            .trim_end()
            .to_string();
        result.push('\n');
        result
    }
}
